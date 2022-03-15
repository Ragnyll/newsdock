use crate::cache;
use crate::newsboat_utils::conf_utils;
use crate::db::{DbError, QueryManager};
use std::process::Command;
use thiserror::Error;

const DEFAULT_LINUX_OPENER: &str = "xdg-open";
const RIFLE: &str = "rifle";

/// Opens the cached file with the opener or if the file is not cache defaults the browser set in
/// the newsboat config
pub fn open(
    url: &str,
    file_opener_program: Option<String>,
    cache_location: Option<String>,
    query_manager: QueryManager,
    video_streamer: Option<String>,
) -> Result<(), OpenerError> {
    log::info!("converting url {url} to id");
    let id = query_manager.get_id_from_url(url)?;
    let cache_location =
        cache_location.unwrap_or_else(|| String::from(cache::DEFAULT_CACHE_LOCATION));
    log::info!("cache location {cache_location:?}");
    if cache::cache_file_ops::check_cache(&id, Some(cache_location.clone())) {
        log::info!("opening {id} from cache");
        open_from_cache(&id, file_opener_program, &cache_location)
    } else {
        log::info!("opening with browser");
        open_with_browser(url, video_streamer)
    }
}

/// This assumes that the file exists. This function should be called through the open public api.
fn open_from_cache(
    id: &str,
    file_opener_program: Option<String>,
    cache_location: &str,
) -> Result<(), OpenerError> {
    let path = cache::cache_file_ops::get_file_matching_basename(id, cache_location);

    if path.is_none() {
        return Err(OpenerError::UnableToOpen);
    }

    match file_opener_program {
        Some(opener) => {
            if opener == RIFLE {
                log::info!("Opening {path:?} using rifle");
                open_from_cache_with_rifle(&path.unwrap())
            } else {
                Err(OpenerError::UnsupportedFileOpener)
            }
        }
        None => {
            log::info!("Opening using system default opener");
            open_from_cache_with_system_default(&path.unwrap())
        }
    }
}

fn open_from_cache_with_rifle(path: &str) -> Result<(), OpenerError> {
    let output = Command::new(RIFLE).arg(path).output().unwrap();
    if !output.stderr.is_empty() {
        return Err(OpenerError::UnableToOpen);
    }

    Ok(())
}

/// Opens the file with the newsboat browser or the system "BROWSER" env var if not provided
fn open_with_browser(url: &str, video_streamer: Option<String>) -> Result<(), OpenerError> {
    if url.contains("https://www.youtube.com") {
        if let Some(vs) = video_streamer {
            log::info!("using video_streamer: {vs} to stream url: {url}");
            let output = Command::new(vs).arg(url).output().unwrap();

            if !output.stderr.is_empty() {
                return Err(OpenerError::UnableToOpen);
            }

            return Ok(());
        }
    }

    let browser = determine_browser()?;
    log::info!("using browser: {browser} to open url: {url}");

    let output = Command::new(browser).arg(url).output().unwrap();

    if !output.stderr.is_empty() {
        return Err(OpenerError::UnableToOpen);
    }

    Ok(())
}

/// opens the url defined by the shell $BROWSER variable
fn determine_browser() -> Result<String, OpenerError> {
    let browser = std::env::var("BROWSER");
    if browser.is_err() {
        return Err(OpenerError::NoBrowserDefined);
    }
    Ok(browser.unwrap())
}

fn open_from_cache_with_system_default(path: &str) -> Result<(), OpenerError> {
    // check for file matching str id
    let output = Command::new(DEFAULT_LINUX_OPENER)
        .arg(path)
        .output()
        .unwrap();

    if !output.stderr.is_empty() {
        return Err(OpenerError::UnableToOpen);
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum OpenerError {
    #[error("Unable to open cached file")]
    UnableToOpen,
    #[error("The specified file opener is not supported")]
    UnsupportedFileOpener,
    #[error("Newsboat Browser config not parseable")]
    UnparsableBrowswer(#[from] conf_utils::NewsboatConfigError),
    #[error("No Browswer variable is defined")]
    NoBrowserDefined,
    #[error("Unable to open url with browser")]
    BrowserOpenError(#[from] DbError),
}
