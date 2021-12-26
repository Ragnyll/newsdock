pub mod schema;

#[derive(Queryable, Debug)]
pub struct RssItem {
    #[allow(dead_code)]
    id: i32,
    #[allow(dead_code)]
    guid: String,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    author: String,
    #[allow(dead_code)]
    url: String,
    #[allow(dead_code)]
    feedurl: String,
    #[allow(dead_code)]
    pubDate: i32,
    #[allow(dead_code)]
    content: String,
    #[allow(dead_code)]
    unread: i32,
    #[allow(dead_code)]
    enclosure_url: Option<String>,
    #[allow(dead_code)]
    enclosure_type: Option<String>,
    #[allow(dead_code)]
    enqueued: i32,
    #[allow(dead_code)]
    flags: Option<String>,
    #[allow(dead_code)]
    deleted: i32,
    #[allow(dead_code)]
    base: String,
    #[allow(dead_code)]
    content_mime_type: String,
}

#[derive(Queryable, Debug)]
pub struct RssFeed {
    #[allow(dead_code)]
    rssurl: String,
    #[allow(dead_code)]
    url: String,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    lastmodified: i32,
    #[allow(dead_code)]
    is_rtl: i32,
    #[allow(dead_code)]
    etag: String,
}
