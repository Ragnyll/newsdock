use newsdock::cache;
use newsdock::conf;
use newsdock::conf::CmdType;
use newsdock::db::QueryManager;
use newsdock::fs;
use newsdock::newsboat_utils::bin_utils;
use std::process;

fn main() {
    let conf = conf::build_conf().unwrap();

    // TODO: replace unwraps at end of decls with anyhow
    let db_location = fs::get_file_location_or_abort(&conf.cache_db_location).unwrap();
    let newsboat_config_location = fs::get_file_location_or_abort(&conf.newsboat_config_location).unwrap();
    let cache_dir = fs::get_dir_or_create(&conf.cache_dir).unwrap();

    let query_manager = match QueryManager::new(&db_location) {
        Ok(qm) => {
            log::info!("Connection established by query manager");
            qm
        }
        Err(e) => {
            log::error!("Failed to connect to DB using query manager: {e}");
            process::exit(exitcode::DATAERR);
        }
    };

    match conf.cmd_type {
        CmdType::Dl => {
            let newsboat_urls_location =
                fs::get_file_location_or_abort(&conf.newsboat_urls_location.unwrap()).unwrap();
            download(conf.skip_refresh.unwrap(), &db_location, &newsboat_urls_location, &newsboat_config_location, &cache_dir, conf.yt_dlp_attempts.unwrap(), query_manager);
        }
        CmdType::Open => {
            println!("Opening")
        }
        CmdType::Clean => {
            println!("Cleaning")
        }
    }
}

fn download(
    skip_refresh: bool,
    db_location: &str,
    newsboat_urls_location: &str,
    newsboat_config_location: &str,
    cache_dir: &str,
    yt_dlp_attempts: u32,
    query_manager: QueryManager) {
    if !skip_refresh {
        match bin_utils::reload_feed_items(
            &db_location,
            &newsboat_urls_location,
            &newsboat_config_location,
        ) {
            Ok(_) => log::info!("cachedb reloaded succesfully"),
            Err(_) => log::error!("Unable to reload rss_items"),
        };
    }

    let item_urls = match query_manager.get_all_cacheable_feed_items() {
        Ok(urls) => urls,
        Err(e) => {
            log::error!("Failed to retrieve urls to download {e}");
            process::exit(exitcode::DATAERR);
        }
    };

    for item in item_urls {
        match cache::downloader::poll_cache(&item, Some(String::from(cache_dir)), yt_dlp_attempts) {
            Ok(_) => log::info!("downloaded: {item}"),
            Err(e) => {
                log::error!("Failed to download \"{item}\": {e}");
            }
        }
    }
}
