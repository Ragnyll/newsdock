mod cli;

use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::convert::From;
use std::str::FromStr;
use thiserror::Error;

/// builds a conf from the aggregate of configuration file arguments and cli arguments
pub fn build_conf() -> Result<Conf, ConfError> {
    let cli_args = cli::get_cli_args();

    merge_confs(None, Conf::from(cli_args))
}

// TODO
/// Merges two confs into a single one with the second overriding the first where applicable
#[allow(dead_code)]
fn merge_confs(_conf_1: Option<Conf>, conf_2: Conf) -> Result<Conf, ConfError> {
    let level = LevelFilter::from_str(&conf_2.log_level)?;
    SimpleLogger::new()
        .with_local_timestamps()
        .with_level(level)
        .init()
        .unwrap();

    Ok(conf_2)
}

pub enum CmdType {
    Dl,
    Open,
    Clean,
    Update,
}

/// The configuration for the newsboat utility
pub struct Conf {
    pub cmd_type: CmdType,
    pub log_level: String,
    pub cache_dir: String,
    pub newsboat_config_location: String,
    pub cache_db_location: String,
    pub open_url: Option<String>,
    pub opener: Option<String>,
    pub newsboat_urls_location: Option<String>,
    pub skip_refresh: Option<bool>,
    pub yt_dlp_attempts: Option<u32>,
}

impl From<cli::Cli> for Conf {
    fn from(cli: cli::Cli) -> Self {
        match cli.command {
            cli::Commands::Dl {
                newsboat_urls_location,
                skip_refresh,
                yt_dlp_attempts,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                log_level,
            } => Self {
                cmd_type: CmdType::Dl,
                log_level,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                open_url: None,
                opener: None,
                newsboat_urls_location: Some(newsboat_urls_location),
                skip_refresh: Some(skip_refresh),
                yt_dlp_attempts: Some(yt_dlp_attempts),
            },
            cli::Commands::Open {
                url,
                opener,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                log_level,
            } => Self {
                cmd_type: CmdType::Open,
                log_level,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                open_url: Some(url),
                opener: Some(opener),
                newsboat_urls_location: None,
                skip_refresh: None,
                yt_dlp_attempts: None,
            },

            cli::Commands::Clean {
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                log_level,
            } => Self {
                cmd_type: CmdType::Clean,
                log_level,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                open_url: None,
                opener: None,
                newsboat_urls_location: None,
                skip_refresh: None,
                yt_dlp_attempts: None,
            },

            cli::Commands::Update {
                newsboat_urls_location,
                skip_refresh,
                yt_dlp_attempts,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                log_level,
            } => Self {
                cmd_type: CmdType::Dl,
                log_level,
                cache_dir,
                newsboat_config_location,
                cache_db_location,
                open_url: None,
                opener: None,
                newsboat_urls_location: Some(newsboat_urls_location),
                skip_refresh: Some(skip_refresh),
                yt_dlp_attempts: Some(yt_dlp_attempts),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfError {
    #[error("loglevel must be in: info, warn, error, trace")]
    InvalidLogLevel(#[from] log::ParseLevelError),
}
