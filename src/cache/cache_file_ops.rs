use std::path::{Path, PathBuf};
use std::fs;

/// returns whether or not the cache contains a file with the specified basename
pub fn check_cache(f_basename: &str, cache_location: Option<&str>) -> bool {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir).join(cache_location.unwrap_or(super::DEFAULT_CACHE_LOCATION));

    for file in fs::read_dir(&path).unwrap() {
        if file.unwrap().path().file_stem().unwrap() == f_basename {
            return true;
        }
    }

    false
}
