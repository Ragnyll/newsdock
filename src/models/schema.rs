table! {
    rss_item (id) {
        id -> Integer,
        guid -> Text,
        title -> Text,
        author -> Text,
        url -> Text,
        feedurl -> Text,
        pubDate -> Integer,
        content -> Text,
        unread -> Integer,
        enclosure_url -> Nullable<Text>,
        enclosure_type -> Nullable<Text>,
        enqueued -> Integer,
        flags -> Nullable<Text>,
        deleted -> Integer,
        base -> Text,
        content_mime_type -> Text,
    }
}

table! {
    rss_feed (rssurl) {
        rssurl -> Text,
        url -> Text,
        title -> Text,
        lastmodified -> Integer,
        is_rtl -> Integer,
        etag -> Text,
    }
}
