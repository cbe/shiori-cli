use disk_persist::DiskPersist;
use reqwest::blocking::Client;
use url::Url;

use crate::structs::shiori_cli::LocalCache;
use crate::{validate, EXIT_CODE_ERROR, EXIT_CODE_SUCCESS};

pub fn logout(http_client: Client, persist: DiskPersist<LocalCache>) {
    if !validate::check_logged_in(&persist) {
        println!("🤷 You're not logged in, nothing to do");
        std::process::exit(EXIT_CODE_ERROR);
    }

    let local_cache = persist.read().unwrap().unwrap();
    let mut logout_url = Url::parse(&local_cache.api_base_url).unwrap();
    logout_url.set_path("api/logout");
    let response = http_client
        .post(logout_url)
        .header("X-Session-Id", local_cache.session_id)
        .send();

    match response {
        Ok(response) => {
            if !response.status().is_success() {
                println!("😞 Request failed");
                println!("Status: {:?}", response.status());
                println!("Message: {:?}", response.text().unwrap());
                std::process::exit(EXIT_CODE_ERROR);
            }

            // Reset the cache
            let data_to_persist = LocalCache {
                api_base_url: "".to_string(),
                session_id: "".to_string(),
                session_expires: "".to_string(),
                username: "".to_string(),
            };
            persist.write(&data_to_persist).unwrap();

            println!("💨 Bye");
            std::process::exit(EXIT_CODE_SUCCESS);
        }
        Err(_error) => {
            println!("😞 Something went wrong");
            std::process::exit(EXIT_CODE_ERROR);
        }
    }
}
