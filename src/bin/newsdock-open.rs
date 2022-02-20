use clap::Parser;
use log::LevelFilter;
use newsdock::opener;
use simple_logger::SimpleLogger;
use std::process;
use std::str::FromStr;

fn main() {
    let args = Args::parse();

    let level: LevelFilter = match LevelFilter::from_str(&args.log_level) {
        Ok(ll) => ll,
        Err(_) => {
            eprintln!("log-level must be one of the following: info, warn, error, trace");
            process::exit(exitcode::USAGE)
        }
    };

    SimpleLogger::new()
        .with_local_timestamps()
        .with_level(level)
        .init()
        .unwrap();

    log::info!("Trying to open \"Fornite Daycare\"");
    let open = opener::open("Fortnite Daycare", Some(String::from("rifle")), None);
    println!("open = {open:?}")
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
    #[clap(long, default_value = ".config/newsboat/urls")]
    newsboat_urls_location: String,

    /// An optional override for the location where the newsboat urls file is stored relative to
    /// the home dir
    #[clap(long, default_value = ".config/newsboat/config")]
    newsboat_config_location: String,

    /// An opitonal override for the location where newsboats db is stored relative to the home_dir
    /// defaults to "/.local/share/newsboat/cache.db"
    #[clap(long, default_value = ".local/share/newsboat/cache.db")]
    cache_db_location: String,

    /// skips the refresh on the newsboatdb
    #[clap(long)]
    skip_refresh: bool,

    /// The amount of times to retry downloads from youtube
    #[clap(long, default_value_t = 20)]
    yt_dlp_attempts: u32,

    #[clap(long, default_value = "error")]
    log_level: String,
}
