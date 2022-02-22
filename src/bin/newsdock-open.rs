use clap::Parser;
use log::LevelFilter;
use newsdock::opener;
use newsdock::db::QueryManager;
use simple_logger::SimpleLogger;
use std::path::Path;
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

    let db_location = get_file_location_or_abort(&args.cache_db_location);
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

    log::info!("Trying to open \"Fortnite Daycare\"");
    let open = opener::open(
        "Fortnite Daycare",
        Some(String::from("rifle")),
        None,
        query_manager,
    );
    println!("open = {open:?}")
}

/// For use of extraction CLI arguments into valid file locations WILL CAUSE EXITS ON INVALID INPUT
fn get_file_location_or_abort(target: &str) -> String {
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => {
            log::error!("Home directory could not be found");
            process::exit(exitcode::OSFILE);
        }
    };

    let t = &home_dir.join(target).into_os_string();

    let t = match t.clone().into_string() {
        Ok(t) => t,
        Err(_) => {
            log::error!("{} is not a valid file location", target);
            process::exit(exitcode::OSFILE);
        }
    };

    if !Path::new(&t).exists() {
        log::error!("{} does not exist", target);
        process::exit(exitcode::OSFILE);
    }

    t
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
