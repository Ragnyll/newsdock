use std::process::Command;
use std::path::Path;
use super::cache_file_ops;
use crate::models::RssItem;
use thiserror::Error;

/// A Meta descriptor to help determine what downloader to use for a given RSS item
enum DownloadType {
    Youtube,
    #[allow(unused)]
    Webpage,
    /// The rss_item's is not of a supported type
    Unsupported,
}

/// checks if the rss_item exists in the cache and downloads it if it is not
pub fn poll_cache(
    rss_item: &RssItem,
    cache_location: &str,
    youtube_dl_attempts: u32,
) -> Result<(), DownloadError> {
    if !cache_file_ops::check_cache(&rss_item.title, cache_location) {
        download(rss_item, cache_location, youtube_dl_attempts)?;
    }

    Ok(())
}

/// Downloads the rss content from RssItem.url
fn download(
    rss_item: &RssItem,
    output_dir: &str,
    youtube_dl_attempts: u32,
) -> Result<(), DownloadError> {
    match determine_download_type(&rss_item.url) {
        DownloadType::Youtube => download_youtube(rss_item, output_dir, youtube_dl_attempts),
        _ => Err(DownloadError::UnsupportedDownloadTypeError),
    }
}

/// downloads the rss item to `{download_base_path}/{rss_item.title}`
fn download_youtube(
    rss_item: &RssItem,
    download_base_path: &str,
    num_retries: u32,
) -> Result<(), DownloadError> {
    let download_output_path = Path::new(download_base_path).join(&rss_item.title);
    Command::new("youtube-dl")
        .arg(&rss_item.url)
        .arg("--retries")
        .arg(num_retries.to_string())
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

    DownloadType::Unsupported
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("newsdock  was unable to download the given rss_item")]
    DownloadError,
    #[error("The rss_item is of an unspoorted DownloadType")]
    UnsupportedDownloadTypeError,
    #[error("The video could not be downloaded from youtube")]
    YoutubeDownloadError,
}
