use crate::db::{DbError, QueryManager};
use crate::models::RssItem;
use std::path::{Path, PathBuf};
use std::fs;

/// A lighter version of RssItem to represent a cached file
#[derive(Debug)]
struct CachedFile {
    id: i32,
    published_date: i32,
}

impl CachedFile {
    fn new(rss_item: RssItem) -> Self {
        Self {
            id: rss_item.id,
            published_date: rss_item.pubDate,
        }
    }
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
pub fn clean_cache(cache_location: &str, query_manager: QueryManager) -> Result<(), DbError> {
    // Group items in cache by the author
    let cached_file_ids = get_cached_file_ids(cache_location);
    let mut cached_rss_items = Vec::with_capacity(cached_file_ids.len());

    for fid in cached_file_ids {
        for item in query_manager.get_rss_item_from_id(fid)? {
            cached_rss_items.push(CachedFile::new(item));
        }
    }

    // if author has items greater than the num allowed by cache evict the oldest until
    // max_items == num_cached by author

    Ok(())
}

pub fn get_cached_file_ids(cache_location: &str) -> Vec<i32> {
    let home_dir: PathBuf = dirs::home_dir().expect("Unable to find home dir while checking cache");
    let path = Path::new(&home_dir).join(cache_location);

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
                .parse::<i32>()
                .unwrap(),
        );
    }

    cached_file_ids
}
