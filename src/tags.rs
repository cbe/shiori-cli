use disk_persist::DiskPersist;
use reqwest::blocking::Client;
use url::Url;

use crate::{
    structs::{shiori_api::ShioriResponseGetTag, shiori_cli::LocalCache},
    validate, EXIT_CODE_ERROR, EXIT_CODE_SUCCESS,
};

pub fn get_tags(http_client: Client, persist: DiskPersist<LocalCache>) {
    if !validate::check_logged_in(&persist) {
        println!("🤷 You're not logged in, please log in first");
        std::process::exit(EXIT_CODE_ERROR);
    }

    let local_cache = persist.read().unwrap().unwrap();

    let mut get_tags_url = Url::parse(&local_cache.api_base_url).unwrap();
    get_tags_url.set_path("api/tags");

    let response = http_client
        .get(get_tags_url)
        .header("X-Session-Id", local_cache.session_id)
        .send();

    match response {
        Ok(response) => {
            if !response.status().is_success() {
                println!("😞 Something went wrong");
                println!("Status: {:?}", response.status());
                println!("Message: {:?}", response.text().unwrap());
                std::process::exit(EXIT_CODE_ERROR);
            }

            let json = serde_json::from_str::<Vec<ShioriResponseGetTag>>(&response.text().unwrap())
                .unwrap();

            let tags = json.iter().map(|tag| &tag.name);
            for tag in tags {
                println!("{}", tag);
            }

            std::process::exit(EXIT_CODE_SUCCESS);
        }
        Err(_error) => {
            println!("😞 Something went wrong");
            std::process::exit(EXIT_CODE_ERROR);
        }
    }
}
