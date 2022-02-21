use crate::cache;
use crate::newsboat_utils::conf_utils;
use std::process::Command;
use thiserror::Error;

const DEFAULT_LINUX_OPENER: &str = "xdg-open";
const RIFLE: &str = "rifle";

/// Opens the cached file with the opener or if the file is not cache defaults the browser set in
/// the newsboat config
pub fn open(
    title: &str,
    file_opener_program: Option<String>,
    cache_location: Option<String>,
) -> Result<(), OpenerError> {
    let cache_location = cache_location.unwrap_or(String::from(cache::DEFAULT_CACHE_LOCATION));
    log::info!("cache location {cache_location:?}");
    if cache::cache_file_ops::check_cache(title, Some(cache_location.clone())) {
        log::info!("opening {title} from cache");
        open_from_cache(title, file_opener_program, &cache_location)
    } else {
        log::info!("opening with browser");
        open_with_browser(title)
    }
}

/// This assumes that the file exists. This function should be called through the open public api.
fn open_from_cache(
    title: &str,
    file_opener_program: Option<String>,
    cache_location: &str,
) -> Result<(), OpenerError> {
    let path = cache::cache_file_ops::get_file_matching_basename(&title, &cache_location);

    if path.is_none() {
        return Err(OpenerError::UnableToOpen);
    }

    match file_opener_program {
        Some(opener) => {
            if opener == String::from(RIFLE) {
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
    Command::new(String::from(RIFLE))
        .arg(path)
        .output()
        .unwrap();
    Ok(())
}

/// Opens the file with the newsboat browser or the system "BROWSER" env var if not provided
fn open_with_browser(_title: &str) -> Result<(), OpenerError> {
    let _browser = determine_browser()?;

    log::error!("opening with browser not yet supported");

    Ok(())
}

/// uses either the browser defined by newsboat or fallsback to the browser defined by $BROWSER
fn determine_browser() -> Result<String, OpenerError> {
    match conf_utils::get_browser()? {
        Some(browser) => Ok(browser),
        None => {
            let browser = std::env::var("BROWSER");
            if browser.is_err() {
                return Err(OpenerError::NoBrowserDefined);
            }
            Ok(browser.unwrap())
        }
    }
}

fn open_from_cache_with_system_default(path: &str) -> Result<(), OpenerError> {
    // check for file matching str title
    Command::new(DEFAULT_LINUX_OPENER)
        .arg(path)
        .output()
        .unwrap();

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
}
