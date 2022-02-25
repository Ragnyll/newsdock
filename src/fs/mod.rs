use std::fs;
use std::process;
use std::path::Path;
/// A set of common file system operation tools

/// For use of extraction CLI arguments into valid file locations WILL CAUSE EXITS ON INVALID INPUT
pub fn get_file_location_or_abort(target: &str) -> String {
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => {
            log::error!("Home directory could not be found");
            process::exit(exitcode::OSFILE);
        }
    };

    let t = &home_dir.join(target).into_os_string();

    let t = match t.clone().into_string() {
        Ok(t) => t,
        Err(_) => {
            log::error!("{} is not a valid file location", target);
            process::exit(exitcode::OSFILE);
        }
    };

    if !Path::new(&t).exists() {
        log::error!("{} does not exist", target);
        process::exit(exitcode::OSFILE);
    }

    t
}

/// For use of extraction CLI arguments into valid file locations WILL CAUSE EXITS ON INVALID INPUT
pub fn get_dir_or_create(target: &str) -> String {
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => {
            log::error!("Home directory could not be found");
            process::exit(exitcode::OSFILE);
        }
    };

    let t = &home_dir.join(target);

    if !t.exists() {
        log::info!("creating cache_dir {t:?}");
        fs::create_dir_all(t).unwrap_or_else(|_| {
            log::error!("{t:?} could not be created");
            process::exit(exitcode::OSFILE);
        });
    }

    t.clone()
        .into_os_string()
        .into_string()
        .unwrap_or_else(|_| {
            log::error!("{t:?} could not be created");
            process::exit(exitcode::OSFILE);
        })
}
