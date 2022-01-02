use clap::Parser;
use newsdock::db::QueryManager;
use newsdock::cache;
use newsdock::newsboat_utils::bin_utils;

fn main() {
    let home_dir = dirs::home_dir().expect("Unable to find home dir");
    let args = Args::parse();

    if !args.skip_refresh {
        bin_utils::reload_feed_items().expect("Unable to reload rss_items");
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
        // TODO: this should be replaced with a logging library later
        let _ = cache::downloader::poll_cache(&item, cache_dir, args.youtube_dl_attempts);
        break;
    }
}

/// A utility for downloading rss_items onto local storage
#[derive(Parser, Debug)]
#[clap(about, version)]
#[readonly::make]
struct Args {
    /// An optional location for the default cache directory
    /// Defaults to ".cache/newsdock/"
    #[clap(long, default_value = ".cache/newsdock/")]
    cache_dir: String,

    /// An optional override for the location where the newsboat urls file is stored relative to
    /// the home dir
    /// Defaults to .config/newsboat/urls"
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
}
