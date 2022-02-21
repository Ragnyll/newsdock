use std::path::{Path, PathBuf};
use std::fs;

/// There should only be one file returned by this function. If more than one is found the cache is
/// in a bad state. In this case only the first is returned
pub fn get_file_matching_basename(f_basename: &str, cache_location: &str) -> Option<String> {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir).join(cache_location);
    let mut matching_files = vec![];

    for file in fs::read_dir(&path).unwrap() {
        if file.as_ref().unwrap().path().file_stem().unwrap() == f_basename {
            matching_files.push(file.unwrap().path().into_os_string().into_string().unwrap());
        }
    }

    if matching_files.len() > 0 {
        return Some(matching_files[0].clone());
    }

    None
}

/// returns whether or not the cache contains a file with the specified basename
pub fn check_cache(f_basename: &str, cache_location: Option<String>) -> bool {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir).join(cache_location.unwrap_or(String::from(crate::cache::DEFAULT_CACHE_LOCATION)));

    log::info!("basename = {f_basename:?}");
    log::info!("path = {path:?}");
    for file in fs::read_dir(&path).unwrap() {
        if file.unwrap().path().file_stem().unwrap() == f_basename {
            log::info!("found {f_basename}");
            return true;
        }
    }

    false
}
