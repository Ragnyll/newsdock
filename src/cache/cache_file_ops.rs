use std::path::{Path, PathBuf};

/// The default cache location relative to the home dir
const DEFAULT_CACHE_LOCATION: &str = ".cache/newdock/";

/// returns whether or not the cache contains a file with the specified basename
pub fn check_cache(fname: &str, cache_location: Option<&str>) -> bool {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    Path::new(&home_dir)
        .join(cache_location.unwrap_or(DEFAULT_CACHE_LOCATION))
        .join(fname)
        .exists()
}
