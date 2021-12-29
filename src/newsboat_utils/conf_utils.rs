/// Provides APIs around the newsboat config and urls file
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use thiserror::Error;

const DEFAULT_URLS_PATH: &str = ".config/newsboat/urls";

/// The Feed Urls and tags from the urls config file
#[derive(Debug)]
pub struct FeedTags {
    pub feed_url: String,
    pub tags: HashSet<String>,
}

impl FeedTags {
    fn new(feed_url: &str, tags: &[String]) -> Self {
        let mut tags_set = HashSet::with_capacity(tags.len());
        tags.iter().for_each(|tag| {
            tags_set.insert(tag.clone());
        });

        FeedTags {
            feed_url: String::from(feed_url),
            tags: tags_set,
        }
    }
}

/// extracts the feed urls and tag from the config file or falls back to the default
pub fn get_feed_urls_tags() -> Vec<FeedTags> {
    let home_dir = find_home_dir().expect("Unable to find home dir to reach urls file");
    let urls_path = format!("{}/{}", home_dir, DEFAULT_URLS_PATH);
    let file = File::open(urls_path).expect("urls file cannot be opened");
    let reader = BufReader::new(file);

    let mut feed_tags = vec![];

    for line in reader.lines() {
        let line = line.expect("unable to read line in newsboat urls file");
        // ignore comment lines that start with `#` or are empty
        if !line.starts_with('#') && !line.is_empty() {
            let split: Vec<String> = line.split(' ').map(String::from).collect();
            feed_tags.push(FeedTags::new(&split[0], &split[1..]));
        }
    }

    feed_tags
}

/// Finds the home directory or errors in the process
fn find_home_dir() -> Result<String, NewsboatConfigError> {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(NewsboatConfigError::HomePathError);
        }
    };

    match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => Err(NewsboatConfigError::HomePathError),
    }
}

#[derive(Error, Debug)]
pub enum NewsboatConfigError {
    #[error("Unable to find the home dir")]
    HomePathError,
}