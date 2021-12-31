use std::process::Command;
use std::path::Path;
use super::cache_file_ops;
use crate::models::RssItem;
use thiserror::Error;

/// The Default number of times to retry downloads from youtube
const DEFAULT_YOUTUBE_RETRIES: u16 = 20;

/// A Meta descriptor to help determine what downloader to use for a given RSS item
enum DownloadType {
    Youtube,
    #[allow(unused)]
    Webpage,
    UnsupportedDownloadType,
}

/// checks if the rss_item exists in the cache and downloads it if it is not
pub fn poll_cache(rss_item: &RssItem, cache_location: Option<&str>) -> Result<(), DownloadError> {
    if !cache_file_ops::check_cache(&rss_item.title, cache_location) {
        download(rss_item)?;
    }

    Ok(())
}

/// Downloads the rss content from RssItem.url
fn download(rss_item: &RssItem) -> Result<(), DownloadError> {
    let download_base_path = dirs::home_dir().expect("Unable to find home dir").join(super::DEFAULT_CACHE_LOCATION).into_os_string().into_string().expect("unable to build cache path");
    match determine_download_type(&rss_item.url) {
        DownloadType::Youtube => {
            return download_youtube(rss_item, &download_base_path, None);
        }
        _ => return Err(DownloadError::UnsupportedDownloadType),
    }
}

/// downloads the rss item to `{download_base_path}/{rss_item.title}`
fn download_youtube(
    rss_item: &RssItem,
    download_base_path: &str,
    num_retries: Option<u16>,
) -> Result<(), DownloadError> {
    let download_output_path = Path::new(download_base_path).join(&rss_item.title);
    Command::new("youtube-dl")
        .arg(&rss_item.url)
        .arg("--retries")
        .arg(num_retries.unwrap_or(DEFAULT_YOUTUBE_RETRIES).to_string())
        .arg("--output")
        .arg(download_output_path)
        .output()
        .unwrap();

    Ok(())
}

/// From the url determines the DownloadType
fn determine_download_type(url: &str) -> DownloadType {
    if url.contains("www.youtube.com") {
        return DownloadType::Youtube;
    }

    DownloadType::UnsupportedDownloadType
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("newsdock  was unable to download the given rss_item")]
    DownloadError,
    #[error("The rss_item is of an unspoorted DownloadType")]
    UnsupportedDownloadType,
    #[error("The video could not be downloaded from youtube")]
    YoutubeDownloadError,
}
