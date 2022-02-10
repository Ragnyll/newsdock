use crate::cache;
use std::process::Command;
use thiserror::Error;

const DEFAULT_LINUX_OPENER: &str = "xdg-open";

pub fn open_from_cache(title: &str, file_opener_program: Option<&str>) -> Result<(), OpenerError> {
    // get the cached file path
    cache::cache_file_ops::check_cache(&"abc", &"abc");

    match file_opener_program {
        Some(opener) => match opener {
            "rifle" => open_from_cache_with_rifle(title),
            _ => Err(OpenerError::UnsupportedFileOpener),
        },
        None => open_from_cache_with_system_default(title),
    }
}

fn open_from_cache_with_rifle(path: &str) -> Result<(), OpenerError> {
    Command::new("rifle")
        .arg(path)
        .output()
        .unwrap();
    Ok(())
}

fn open_from_cache_with_system_default(path: &str) -> Result<(), OpenerError> {
    // check for file matching str title
    Command::new(DEFAULT_LINUX_OPENER)
        .arg(path)
        .output()
        .unwrap();

    Ok(())
}

#[derive(Error, Debug)]
pub enum OpenerError {
    #[error("Unable to open cached file")]
    UnableToOpen,
    #[error("The specified file opener is not supported")]
    UnsupportedFileOpener,
}
