use crate::db::{DbError, QueryManager};
use crate::models::RssItem;
use crate::newsboat_utils::conf_utils;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use thiserror::Error;

/// A lighter version of RssItem to represent a cached file
#[derive(Debug, Eq)]
struct CachedFile {
    id: i32,
    published_date: i32,
}

/// For sorting a CachedFile by the published_date
impl Ord for CachedFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.published_date.cmp(&other.published_date)
    }
}

impl PartialOrd for CachedFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CachedFile {
    fn eq(&self, other: &Self) -> bool {
        self.published_date == other.published_date
    }
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
pub fn clean_cache(cache_location: &str, query_manager: QueryManager) -> Result<(), CacheError> {
    let cached_file_by_feed = get_cached_file_by_feed(cache_location, query_manager)?;
    // if feedurl has items greater than the num allowed by cache evict the oldest until
    // max_items == num_cached by feedutl
    let max_items = conf_utils::get_max_items()? as usize;
    for mut cached_ids in cached_file_by_feed {
        cached_ids.1.sort_unstable();
        cached_ids.1.reverse();
        while cached_ids.1.len() > max_items {
            // remove files from cache and the cached_ids mem object
            let file_to_remove = get_file_matching_basename(
                &cached_ids.1.pop().unwrap().id.to_string(),
                cache_location,
            )
            .unwrap();
            log::info!("removing {file_to_remove}");
            fs::remove_file(file_to_remove)?;
        }
    }

    Ok(())
}

fn get_cached_file_by_feed(
    cache_location: &str,
    query_manager: QueryManager,
) -> Result<HashMap<String, Vec<CachedFile>>, CacheError> {
    // Group items in cache by the author
    let cached_file_ids = get_cached_file_ids(cache_location);
    // TODO: would a BTree map be better here
    let mut cached_file_by_author: HashMap<String, Vec<CachedFile>> = HashMap::new();

    for fid in cached_file_ids {
        for item in query_manager.get_rss_item_from_id(fid)? {
            if !cached_file_by_author.contains_key(&item.feedurl) {
                cached_file_by_author.insert(item.feedurl.clone(), vec![]);
            }
            let feedurl_cache = cached_file_by_author.get_mut(&item.feedurl).unwrap();
            feedurl_cache.push(CachedFile::new(item));
        }
    }

    Ok(cached_file_by_author)
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

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Unable to query db")]
    QueryError(#[from] DbError),
    #[error("Unable to delete cached file")]
    CacheRemoveError(#[from] std::io::Error),
    #[error("Invalid newsboat conf file")]
    ConfError(#[from] conf_utils::NewsboatConfigError),
}
