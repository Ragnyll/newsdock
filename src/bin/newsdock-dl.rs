use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}

/// A utility for downloading rss_items onto local storage
#[derive(Parser, Debug)]
#[clap(about, version)]
struct Args {
    /// An optional location for the default cache directory
    /// Defaults to ".cache/newsdock/"
    #[clap(long)]
    cache_dir: Option<String>,

    /// An optional override for the location where the newsboat urls file is stored relative to
    /// the home dir
    /// Defaults to .config/newsboat/urls"
    #[clap(short, long)]
    newsboat_urls_location: Option<String>,

    /// An opitonal override for the location where newsboats db is stored relative to the home_dir
    /// defaults to "/.local/share/newsboat/cache.db"
    #[clap(long)]
    cache_db_location: Option<String>,
}
