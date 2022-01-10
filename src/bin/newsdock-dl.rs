use clap::Parser;
use log::LevelFilter;
use newsdock::db::QueryManager;
use newsdock::cache;
use newsdock::newsboat_utils::bin_utils;
use simple_logger::SimpleLogger;
use std::process;
use std::str::FromStr;

fn main() {
    let args = Args::parse();
    // TODO: an invalid level should call the help method. so this should go somewhere else
    let level: LevelFilter = LevelFilter::from_str(&args.log_level).expect("invalid log level");
    SimpleLogger::new().with_local_timestamps().with_level(level).init().unwrap();

    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => {
            log::error!("Home directory could not be found");
            process::exit(exitcode::DATAERR);
        }
    };

    if !args.skip_refresh {
        bin_utils::reload_feed_items().expect("unable to reload rss_items");
    }

    let db_location = &home_dir
        .join(args.cache_db_location)
        .into_os_string()
        .into_string()
        .unwrap();

    let cache_dir = &home_dir
        .join(args.cache_dir)
        .into_os_string()
        .into_string()
        .unwrap();

    let query_manager = QueryManager::new(db_location);

    let item_urls = query_manager.get_all_cacheable_feed_items();

    for item in item_urls {
        // todo: this should be replaced with a logging library later instead of potentially
        // swallowing a result
        let _ = cache::downloader::poll_cache(&item, cache_dir, args.youtube_dl_attempts);
    }
}

/// A utility for downloading rss_items onto local storage
#[derive(Parser, Debug)]
#[clap(about, version)]
#[readonly::make]
struct Args {
    /// An optional location for the default cache directory
    #[clap(long, default_value = ".cache/newsdock/")]
    cache_dir: String,

    /// An optional override for the location where the newsboat urls file is stored relative to
    /// the home dir
    #[clap(short, long)]
    newsboat_urls_location: Option<String>,

    /// An opitonal override for the location where newsboats db is stored relative to the home_dir
    /// defaults to "/.local/share/newsboat/cache.db"
    #[clap(long, default_value = ".local/share/newsboat/cache.db")]
    cache_db_location: String,

    /// skips the refresh on the newsboatdb
    #[clap(long)]
    skip_refresh: bool,

    /// The amount of times to retry downloads from youtube
    #[clap(long, default_value_t = 20)]
    youtube_dl_attempts: u32,

    #[clap(long, default_value = "error")]
    log_level: String
}
