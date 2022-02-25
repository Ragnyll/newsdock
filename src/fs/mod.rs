use std::fs;
use std::path::Path;
use thiserror::Error;

/// A set of common file system operation tools

/// For use of extraction CLI arguments into valid file locations WILL CAUSE EXITS ON INVALID INPUT
pub fn get_file_location_or_abort(target: &str) -> Result<String, FsError> {
    let home_dir = match dirs::home_dir() {
        Some(hd) => hd,
        None => return Err(FsError::HomeDirNotFound),
    };
    let t = &home_dir.join(target).into_os_string();

    if !Path::new(&t).exists() {
        return Err(FsError::FileNotFound);
    }

    match t.clone().into_string() {
        Ok(fl) => Ok(fl),
        Err(_) => Err(FsError::InvalidUnicode),
    }
}

/// For use of extraction CLI arguments into valid file locations WILL CAUSE EXITS ON INVALID INPUT
pub fn get_dir_or_create(target: &str) -> Result<String, FsError> {
    let home_dir = match dirs::home_dir() {
        Some(hd) => hd,
        None => return Err(FsError::HomeDirNotFound),
    };
    let t = &home_dir.join(target);

    if !t.exists() {
        log::info!("creating cache_dir {t:?}");
        fs::create_dir_all(t)?;
    }

    match t.clone().into_os_string().into_string() {
        Ok(cd) => Ok(cd),
        Err(_) => Err(FsError::FileNotFound),
    }
}

#[derive(Error, Debug)]
pub enum FsError {
    #[error("Unable to find the home dir")]
    HomeDirNotFound,
    #[error("File not found")]
    FileNotFound,
    #[error("Invalid Unicode in file path")]
    InvalidUnicode,
    #[error("IoError on cache dir creation")]
    CacheCreationFailed(#[from] std::io::Error),
}
