// Note: this requires the `derive` feature

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A cache management
#[derive(Parser, Debug)]
#[clap(name = "newsdock")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clones repos
    #[clap(arg_required_else_help = true)]
    Open {
        /// The url to download
        remote: String,
    },

    /// Updates the cache with all the items to download
    Dl,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Open { remote } => {
            println!("Cloning {}", remote);
        }
        Commands::Dl => {
            println!("Downloading the cache");
        }
    }
}
