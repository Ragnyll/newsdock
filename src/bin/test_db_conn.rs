extern crate diesel;
extern crate newsdock;

use newsdock::db::QueryManager;

fn main() {
    let query_manager = QueryManager::new("cache.db");
    let cacheable_items = query_manager.get_all_cacheable_feed_items();
    println!("{:?}", cacheable_items);
}
