use diesel::prelude::*;
use crate::models::RssItem;
use crate::models::schema::rss_item::dsl::*;
use crate::newsboat_utils::conf_utils;
use thiserror::Error;

const CACHE_TAG: &str = "cache";

/// Maintains the Database connection and runs canned queries
pub struct QueryManager {
    connection: SqliteConnection,
}

impl QueryManager {
    /// Creates a new QueryManager connected to the given database
    pub fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        Ok(Self {
            connection: SqliteConnection::establish(database_url)?,
        })
    }

    /// Returns all RssItem(s) that have the cache tag
    pub fn get_all_cacheable_feed_items(self) -> Result<Vec<RssItem>, DbError> {
        let mut cacheable_feed_urls: Vec<String> = vec![];

        for url_conf in conf_utils::get_feed_urls_tags()?.into_iter() {
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
                    .load::<RssItem>(&self.connection)?,
            );
        }

        Ok(results)
    }

    /// Given a unique url find the rss article title that is associated to it
    pub fn get_title_from_url(&self, url_search: &str) -> Result<String, DbError> {
        Ok(rss_item
            .filter(url.eq(url_search))
            .load::<RssItem>(&self.connection)?[0]
            .title
            .clone())
    }

    /// Given a unique url find the rss article title that is associated to it
    pub fn get_id_from_url(&self, url_search: &str) -> Result<String, DbError> {
        Ok(rss_item
            .filter(url.eq(url_search))
            .load::<RssItem>(&self.connection)?[0]
            .id
            .to_string())
    }

    /// Given a unique title find the rss article that is associated to it
    pub fn get_url_from_title(&self, title_search: &str) -> Result<String, DbError> {
        Ok(rss_item
            .filter(title.eq(title_search))
            .load::<RssItem>(&self.connection)?[0]
            .url
            .clone())
    }

    pub fn get_rss_item_from_id(&self, id_search: i32) -> Result<Vec<RssItem>, DbError> {
        Ok(rss_item
            .filter(id.eq(id_search))
            .load::<RssItem>(&self.connection)?)
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Unable to find the home dir")]
    QueryError(#[from] diesel::result::Error),
    #[error("Unable to read newsboat urls file")]
    ConfError(#[from] conf_utils::NewsboatConfigError),
}
