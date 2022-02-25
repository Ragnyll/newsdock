mod cli;

use std::convert::From;

/// builds a conf from the aggregate of configuration file arguments and cli arguments
pub fn build_conf() -> Conf {
    let cli_args = cli::get_cli_args();

    Conf::from(cli_args)
}

/// Merges two confs into a single one with the second overriding the first where applicable
fn merge_confs(conf_1: Conf, conf_2: Conf) {}

pub enum CmdType {
    Dl,
    Open,
    Clean,
}

pub struct Conf {
    pub cmd_type: CmdType,
}

impl From<cli::Cli> for Conf {
    fn from(cli: cli::Cli) -> Self {
        match cli.command {
            cli::Commands::Dl {newsboat_urls_location, skip_refresh, yt_dlp_attempts} => {
                Self {
                    cmd_type: CmdType::Dl
                }
            },
            cli::Commands::Open { url } => {
                Self {
                    cmd_type: CmdType::Open
                }
            },

            cli::Commands::Clean { } => {
                Self {
                    cmd_type: CmdType::Clean
                }
            }
        }
    }
}
