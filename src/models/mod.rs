#![allow(dead_code)]
pub mod schema;

#[derive(Queryable, Debug)]
pub struct RssItem {
    id: i32,
    guid: String,
    pub title: String,
    author: String,
    url: String,
    feedurl: String,
    pubDate: i32,
    content: String,
    unread: i32,
    enclosure_url: Option<String>,
    enclosure_type: Option<String>,
    enqueued: i32,
    flags: Option<String>,
    deleted: i32,
    base: String,
    content_mime_type: String,
}

#[derive(Queryable, Debug)]
pub struct RssFeed {
    rssurl: String,
    url: String,
    title: String,
    lastmodified: i32,
    is_rtl: i32,
    etag: String,
}
