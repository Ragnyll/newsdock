![logo](./assets/newsdock_logo.png)

A process to cache unread rss content as files for [newsboat](https://newsboat.org/) for offline viewing.

**Current Project State**: This project is in its alpha stage. That means it works in my basic usage with the options that have been provided.
 Any other functionality can be prioritized based off any user feedback or if I decide I need it.

## Demo

![demo](./assets/demo.gif)

## Install Instructions

_Prerequisites_:

Download the prerequisite from your operating system's respective package manager.
```
newsboat
youtube-dl
rifle
```

It is also required that you run newsboat once to initialize its cache db.

_Installation_:
```
cd newsdock
cargo install --path=.
```

Run `newsdock` for a summary of commands but you will likely use only use `newsdock update` in some sort of daemon process and `newsdock open` in newsboat.

In order to use the opener in newsboat to check the cache first add the following to your newsboat config file (which is likely at `~/.config/newsboat/config`):
```
browser "newsdock open %u"
```

In your newsboat urls (default in `~/.config/newsboat/urls`) file you will need to add the cache tag for any feeds you wish to cache.
```
# Youtube subscriptions
## Rust
https://www.youtube.com/feeds/videos.xml?channel_id=UCaYhcUwRBNscFNUKTjgPFiA youtube programming rust cache
```

Now that everything is configured try it out:
1. Run `newsdock update`
1. Run `newsboat`
1. Open something that's been cached.
