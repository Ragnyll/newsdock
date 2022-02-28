# Newsdock
A process to pre-cache certain files for newsboat so that can be viewed locally without an internet connection.

NOTE: This project has not yet reached its MVP stage. Feel free to contribute, or track [the downloader project until it is ready](https://github.com/Ragnyll/newsdock/projects/1)

## Install Instructions

_Prerequisites_:

Download the prerequisite from your operating system's respective package manager.
```
newsboat
youtube-dl
```

it is also required that you run newsboat once to initialize its cache db.

_Installation_:
```
cargo install --path=.
```

Run `newsdock` for a summary of commands but you will likely use only use `newsdock update` in some sort of daemon process and `newsdock open` in newsboat.

In order to use the opener in newsboat to check the cache first add the following to your newsboat config file (which is likely at `~/.config/newsboat/config`):
```
browser "newsdock open %u"
```


## How it works
![newsdock erd](assets/newsdock_erd.png)

The `newsdock_downloader` looks for all rss_items that have the tag: `cache` and attempts to download them from the corresponding `external service` if there is a valid `cache_protocol` defined.

The `newsdock_opener` looks for a file matching the `rss_item` in the `cache_dir`.
 If the file is not in the cache it will fall back to the default newsboat opening program.

