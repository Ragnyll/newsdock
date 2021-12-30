use super::cache_file_ops;
use crate::models::RssItem;
use thiserror::Error;

enum DownloadType {
    Youtube,
    #[allow(unused)]
    Webpage
}

/// checks if the rss_item exists in the cache and downloads it if it is not
pub fn poll_cache(rss_item: &RssItem, cache_location: Option<&str>) -> Result<(), DownloadError>{

    if !cache_file_ops::check_cache(&rss_item.title, cache_location) {
            download()?;
    }

    Ok(())

}

fn download() -> Result<(), DownloadError> {
    match determine_download_type() {
        DownloadType::Youtube => (),
        _ => return Err(DownloadError::UnsupportedDownloadType)
    }

    Ok(())
}

fn download_youtube(rss_item: RssItem) -> Result<(), DownloadError> {
    if 1 == 2 {
        return Err(DownloadError::YoutubeDownloadError);
    }

    Ok(())
}

fn determine_download_type() -> DownloadType {
    DownloadType::Youtube
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
