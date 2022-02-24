use crate::models::RssItem;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::path::Path;
use super::cache_file_ops;
use thiserror::Error;

/// A Meta descriptor to help determine what downloader to use for a given RSS item
enum DownloadType {
    Youtube,
    Webpage,
    /// The rss_item's is not of a supported type
    // Currently Unused. I cant think of a case where the page wont at least have html. I think
    // this enum value still holds value though
    #[allow(unused)]
    Unsupported,
}

/// checks if the rss_item exists in the cache and downloads it if it is not
pub fn poll_cache(
    rss_item: &RssItem,
    cache_location: Option<String>,
    yt_dlp_attempts: u32,
) -> Result<(), DownloadError> {
    if !cache_file_ops::check_cache(&rss_item.id.to_string(), cache_location.clone()) {
        log::info!(
            "rss_item {} {} not found in cache. Downloading",
            rss_item.id,
            rss_item.title
        );
        download(rss_item, &cache_location.unwrap(), yt_dlp_attempts)?;
    }

    Ok(())
}

/// Downloads the rss content from RssItem.url
fn download(
    rss_item: &RssItem,
    output_dir: &str,
    yt_dlp_attempts: u32,
) -> Result<(), DownloadError> {
    match determine_download_type(&rss_item.url) {
        DownloadType::Youtube => {
            log::info!("Downloading using yt-dlp Strategy");
            download_youtube(rss_item, output_dir, yt_dlp_attempts)
        }
        DownloadType::Webpage => {
            log::info!("Downloading using Webpage Download Strategy");
            download_webpage(rss_item, output_dir)
        }
        _ => {
            log::error!("Unable to determine a download strategy");
            Err(DownloadError::UnsupportedDownloadTypeError)
        }
    }
}

/// Downloads the webpage
fn download_webpage(rss_item: &RssItem, output_base_path: &str) -> Result<(), DownloadError> {
    let download_output_path =
        Path::new(output_base_path).join(format!("{}.html", &rss_item.id.to_string()));
    let resp = reqwest::blocking::get(&rss_item.url)?.bytes()?;
    let mut ofile = File::create(download_output_path)?;
    ofile.write_all(&resp)?;
    Ok(())
}

/// downloads the rss item to `{output_base_path}/{rss_item.id}`
fn download_youtube(
    rss_item: &RssItem,
    output_base_path: &str,
    num_retries: u32,
) -> Result<(), DownloadError> {
    let download_output_path = Path::new(output_base_path).join(&rss_item.id.to_string());
    Command::new("yt-dlp")
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
    DownloadType::Webpage
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("newsdock  was unable to download the given rss_item")]
    DownloadError,
    #[error("The rss_item is of an unspoorted DownloadType")]
    UnsupportedDownloadTypeError,
    #[error("The video could not be downloaded from youtube")]
    YoutubeDownloadError,
    #[error("The webpage could not be downloaded")]
    WebpageDownloadError(#[from] reqwest::Error),
    #[error("The downloaded webpage conent could not be written to output")]
    DownloadWriteError(#[from] std::io::Error),
}
