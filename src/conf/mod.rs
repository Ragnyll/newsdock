mod cli;

use log::LevelFilter;
use std::convert::From;
use std::str::FromStr;
use thiserror::Error;

/// builds a conf from the aggregate of configuration file arguments and cli arguments
pub fn build_conf() -> Result<Conf, ConfError> {
    let cli_args = cli::get_cli_args();
    LevelFilter::from_str(&cli_args.log_level)?;

    Ok(Conf::from(cli_args))
}

// TODO
/// Merges two confs into a single one with the second overriding the first where applicable
#[allow(dead_code)]
#[allow(unused_variables)]
fn merge_confs(conf_1: Conf, conf_2: Conf) {}

pub enum CmdType {
    Dl,
    Open,
    Clean,
}

/// The configuration for the newsboat utility
pub struct Conf {
    pub cmd_type: CmdType,
    pub log_level: String,
    pub cache_dir: String,
    pub newsboat_config_location: String,
    pub cache_db_location: String,
    pub open_url: Option<String>,
    pub newsboat_urls_location: Option<String>,
    pub skip_refresh: Option<bool>,
    pub yt_dlp_attempts: Option<u32>,
}

impl From<cli::Cli> for Conf {
    fn from(cli: cli::Cli) -> Self {
        match cli.command {
            cli::Commands::Dl {newsboat_urls_location, skip_refresh, yt_dlp_attempts} => {
                Self {
                    cmd_type: CmdType::Dl,
                    log_level: cli.log_level,
                    cache_dir: cli.cache_dir,
                    newsboat_config_location: cli.newsboat_config_location,
                    cache_db_location:  cli.cache_db_location,
                    open_url: None,
                    newsboat_urls_location: Some(newsboat_urls_location),
                    skip_refresh: Some(skip_refresh),
                    yt_dlp_attempts: Some(yt_dlp_attempts)
                }
            },
            cli::Commands::Open { url } => {
                Self {
                    cmd_type: CmdType::Open,
                    log_level: cli.log_level,
                    cache_dir: cli.cache_dir,
                    newsboat_config_location: cli.newsboat_config_location,
                    cache_db_location:  cli.cache_db_location,
                    open_url: Some(url),
                    newsboat_urls_location: None,
                    skip_refresh: None,
                    yt_dlp_attempts: None
                }
            },

            cli::Commands::Clean { } => {
                Self {
                    cmd_type: CmdType::Clean,
                    log_level: cli.log_level,
                    cache_dir: cli.cache_dir,
                    newsboat_config_location: cli.newsboat_config_location,
                    cache_db_location:  cli.cache_db_location,
                    open_url: None,
                    newsboat_urls_location: None,
                    skip_refresh: None,
                    yt_dlp_attempts: None
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfError {
    #[error("loglevel must be in: info, warn, error, trace")]
    InvalidLogLevel(#[from] log::ParseLevelError)
}
