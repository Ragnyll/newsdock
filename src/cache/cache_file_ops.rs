use std::path::{Path, PathBuf};

/// returns whether or not the cache contains a file with the specified basename
/// TODO: check only file basename
pub fn check_cache(fname: &str, cache_location: Option<&str>) -> bool {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    Path::new(&home_dir)
        .join(cache_location.unwrap_or(super::DEFAULT_CACHE_LOCATION))
        .join(fname)
        .exists()
}
