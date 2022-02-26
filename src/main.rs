use anyhow::Result;
use newsdock::cache;
use newsdock::conf;
use newsdock::conf::CmdType;
use newsdock::db::{DbError, QueryManager};
use newsdock::fs;
use newsdock::newsboat_utils::bin_utils;
use newsdock::opener;

fn main() -> Result<()> {
    let conf = conf::build_conf().unwrap();

    let db_location = fs::get_file_location_or_abort(&conf.cache_db_location)?;
    let newsboat_config_location = fs::get_file_location_or_abort(&conf.newsboat_config_location)?;
    let cache_dir = fs::get_dir_or_create(&conf.cache_dir)?;

    let query_manager = QueryManager::new(&db_location)?;

    match conf.cmd_type {
        CmdType::Dl => {
            let newsboat_urls_location =
                fs::get_file_location_or_abort(&conf.newsboat_urls_location.unwrap())?;
            download(
                conf.skip_refresh.unwrap(),
                &db_location,
                &newsboat_urls_location,
                &newsboat_config_location,
                &cache_dir,
                conf.yt_dlp_attempts.unwrap(),
                query_manager,
            )?;
        }
        CmdType::Open => {
            open(&conf.open_url.unwrap(), "rifle", &cache_dir, query_manager)?;
        }
        CmdType::Clean => {
            eprintln!("Cache Clean not yet implemented")
        }
    }

    Ok(())
}

fn open(
    url: &str,
    opener_bin: &str,
    cache_dir: &str,
    query_manager: QueryManager,
) -> Result<(), opener::OpenerError> {
    opener::open(
        url,
        Some(String::from(opener_bin)),
        Some(String::from(cache_dir)),
        query_manager,
    )
}

fn download(
    skip_refresh: bool,
    db_location: &str,
    newsboat_urls_location: &str,
    newsboat_config_location: &str,
    cache_dir: &str,
    yt_dlp_attempts: u32,
    query_manager: QueryManager,
) -> Result<(), DbError> {
    if !skip_refresh {
        match bin_utils::reload_feed_items(
            db_location,
            newsboat_urls_location,
            newsboat_config_location,
        ) {
            Ok(_) => log::info!("cachedb reloaded succesfully"),
            Err(_) => log::error!("Unable to reload rss_items"),
        };
    }

    let item_urls = query_manager.get_all_cacheable_feed_items()?;

    for item in item_urls {
        match cache::downloader::poll_cache(&item, Some(String::from(cache_dir)), yt_dlp_attempts) {
            Ok(_) => log::info!("downloaded: {item}"),
            Err(e) => {
                log::error!("Failed to download \"{item}\": {e}");
            }
        }
    }

    Ok(())
}
