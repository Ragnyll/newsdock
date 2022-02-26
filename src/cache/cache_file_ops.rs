use std::path::{Path, PathBuf};
use std::fs;

struct CachedFile {
    author: String,
    id: u32,
    //TODO: probaly wanna use actually date time, (blegh)
    published_date: String,
}

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

    if !matching_files.is_empty() {
        return Some(matching_files[0].clone());
    }

    None
}

/// returns whether or not the cache contains a file with the specified basename
pub fn check_cache(f_basename: &str, cache_location: Option<String>) -> bool {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir)
        .join(cache_location.unwrap_or_else(|| String::from(crate::cache::DEFAULT_CACHE_LOCATION)));

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

/// cleans the cache of items that do not fit the cache rule
pub fn clean_cache() {
    // Group items in cache by the author

    // if author has items greater than the num allowed by cache evict the oldest until
    // max_items == num_cached by author
}

pub fn get_cached_file_ids(cache_location: Option<String>) -> Vec<u32> {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir)
        .join(cache_location.unwrap_or_else(|| String::from(crate::cache::DEFAULT_CACHE_LOCATION)));

    let mut cached_file_ids = vec![];
    for file in fs::read_dir(&path).unwrap() {
        cached_file_ids.push(
            file.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
    }

    cached_file_ids
}
