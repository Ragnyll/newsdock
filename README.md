# Newsdock
A process to pre-cache certain files for newsboat so that can be viewed locally without an internet connection.

## Install Instructions

_Prerequisites_:

Download the prerequisite from your operating system's respective package manager.
```
newsboat
youtube-dl
```

_Installation_:
```
cargo install
```

## Usage
`newsdock-dl` downloads items from the newsboat config file tagged with `cache` to the newsdock cache dir.

```
$ newsdock-dl --help
newsdock 0.1.0
A utility for downloading rss_items onto local storage

USAGE:
    newsdock-dl [OPTIONS]

OPTIONS:
        --cache-db-location <CACHE_DB_LOCATION>
            An opitonal override for the location where newsboats db is stored relative to the
            home_dir defaults to "/.local/share/newsboat/cache.db" [default:
            .local/share/newsboat/cache.db]

        --cache-dir <CACHE_DIR>
            An optional location for the default cache directory Defaults to ".cache/newsdock/"
            [default: .cache/newsdock/]

    -h, --help
            Print help information

    -n, --newsboat-urls-location <NEWSBOAT_URLS_LOCATION>
            An optional override for the location where the newsboat urls file is stored relative to
            the home dir Defaults to .config/newsboat/urls"

        --skip-refresh
            skips the refresh on the newsboatdb

    -V, --version
            Print version information

        --youtube-dl-attempts <YOUTUBE_DL_ATTEMPTS>
            The amount of times to retry downloads from youtube [default: 20]
```

## How it works
![newsdock erd](assets/newsdock_erd.png)

The `newsdock_downloader` looks for all rss_items that have the tag: `cache` and attempts to download them from the corresponding `external service` if there is a valid `cache_protocol` defined.

The `newsdock_opener` looks for a file matching the `rss_item` in the `cache_dir`.
 If the file is not in the cache it will fall back to the default newsboat opening program.

