extern crate diesel;
extern crate newsdock;

use newsdock::db::QueryManager;
use newsdock::newsboat_utils::bin_utils;

fn main() {
    let query_manager = QueryManager::new("cache.db");
    let cacheable_items = query_manager.get_all_cacheable_feed_items();
    println!("{:?}", cacheable_items);
    bin_utils::reload_feed_items().expect("Unable to reload rss_items");
}
