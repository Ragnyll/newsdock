use diesel::prelude::*;
use crate::models::RssItem;
use crate::models::schema::rss_item::dsl::*;
use crate::newsboat_utils::conf_utils;

const CACHE_TAG: &str = "cache";

/// Maintains the Database connection and runs canned queries
pub struct QueryManager {
    connection: SqliteConnection,
}

impl QueryManager {
    /// Creates a new QueryManager connected to the given database
    pub fn new(database_url: &str) -> Self {
        Self {
            connection: SqliteConnection::establish(database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        }
    }

    /// Returns all RssItem(s) that have the cache tag
    pub fn get_all_cacheable_feed_items(self) -> Vec<RssItem> {
        let mut cacheable_feed_urls: Vec<String> = vec![];

        for url_conf in conf_utils::get_feed_urls_tags().into_iter() {
            if url_conf.tags.contains(CACHE_TAG) {
                cacheable_feed_urls.push(url_conf.feed_url);
            }
        }

        let mut results = Vec::with_capacity(cacheable_feed_urls.len());

        for cache_feed_url in cacheable_feed_urls {
            results.append(
                &mut rss_item
                    .filter(feedurl.eq(cache_feed_url))
                    .filter(unread.eq(1))
                    .load::<RssItem>(&self.connection)
                    .unwrap(),
            );
        }

        results
    }
}
