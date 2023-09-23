use disk_persist::DiskPersist;

use crate::structs::shiori_cli::LocalCache;

pub fn check_logged_in(persist: &DiskPersist<LocalCache>) -> bool {
    let local_cache = persist.read().unwrap();

    match local_cache {
        Some(local_cache) => {
            return !local_cache.session_id.is_empty();
        }
        None => {
            return false;
        }
    };
}
