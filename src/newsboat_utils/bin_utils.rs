/// Provides and api wrapper around the system installed newsboat binary
use std::process::Command;
use thiserror::Error;

/// Reloads the rss_items
/// NOTE: This can fail without an errorcode. If there is not internet connection it will exit
/// immediately. The best way of mitigating this risk is by checking the internet connection before
/// attmpting this reload.
pub fn reload_feed_items(newsboat_cache_location: &str, url_file_location: &str, newsboat_config_location: &str) -> Result<(), NewsboatBinError> {
    Command::new("newsboat")
        .arg("--execute=reload")
        .arg(format!("--cache-file={}", newsboat_cache_location))
        .arg(format!("--url-file={}", url_file_location))
        .arg(format!("--config-file={}", newsboat_config_location))
        .output()
        .unwrap();

    Ok(())
}

#[derive(Error, Debug)]
pub enum NewsboatBinError {
    #[error("newsboat was unable to reload its rss_items")]
    ReloadError,
}
