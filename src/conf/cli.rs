use clap::{Parser, Subcommand};

/// A cache management tool for newsboat
#[derive(Parser, Debug)]
#[clap(name = "newsdock")]
pub struct Cli {
    #[clap(long, default_value = "error")]
    log_level: String,

    #[clap(subcommand)]
    pub command: Commands,

    /// An optional location for the default cache directory
    #[clap(long, default_value = ".cache/newsdock/")]
    cache_dir: String,

    /// An optional override for the location where the newsboat urls file is stored relative to
    /// the home dir
    #[clap(long, default_value = ".config/newsboat/config")]
    newsboat_config_location: String,

    /// An opitonal override for the location where newsboats db is stored relative to the home_dir
    /// defaults to "/.local/share/newsboat/cache.db"
    #[clap(long, default_value = ".local/share/newsboat/cache.db")]
    cache_db_location: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Opens a url by prefferring the cache check first
    #[clap(arg_required_else_help = true)]
    Open {
        /// The url to download
        url: String,
    },

    /// Updates the cache with all the items to download
    Dl {
        /// An optional override for the location where the newsboat urls file is stored relative to
        /// the home dir
        #[clap(long, default_value = ".config/newsboat/urls")]
        newsboat_urls_location: String,

        /// skips the refresh on the newsboatdb
        #[clap(long)]
        skip_refresh: bool,

        /// The amount of times to retry downloads from youtube
        #[clap(long, default_value_t = 20)]
        yt_dlp_attempts: u32,
    },

    /// Cleans the cache following the an oldest first eviction policy
    Clean {

    }
}

pub fn get_cli_args() -> Cli {
    return Cli::parse();
}


