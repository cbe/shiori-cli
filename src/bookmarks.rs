use disk_persist::DiskPersist;
use reqwest::blocking::Client;
use url::Url;

use crate::structs::shiori_api::{
    ShioriRequestTag, ShioriResponseAddBookmark, ShioriResponseGetBookmarks,
};
use crate::structs::shiori_cli::LocalCache;
use crate::{validate, EXIT_CODE_ERROR, EXIT_CODE_SUCCESS};

pub fn add(http_client: Client, persist: DiskPersist<LocalCache>, tags: Vec<String>, url: String) {
    if !validate::check_logged_in(&persist) {
        println!("🫤 You're not logged in");
        std::process::exit(EXIT_CODE_ERROR);
    }

    if Url::parse(&url).is_err() {
        println!("🔗 <URL> must be a valid URL, got {:?}", url);
        std::process::exit(EXIT_CODE_ERROR);
    }

    let local_cache = persist.read().unwrap().unwrap();
    let mut add_bookmark_url = Url::parse(&local_cache.api_base_url).unwrap();
    add_bookmark_url.set_path("api/bookmarks");
    let shiori_tags: Vec<_> = tags
        .iter()
        .flat_map(|tag| -> Vec<_> { tag.split(',').collect() })
        .map(|tag| ShioriRequestTag {
            name: tag.to_string(),
        })
        .collect();

    let payload = serde_json::json!({
        "url": url,
        "tags": shiori_tags,
        "createArchive": false,
    });

    let response = http_client
        .post(add_bookmark_url)
        .header("X-Session-Id", local_cache.session_id)
        .json(&payload)
        .send();

    match response {
        Ok(response) => {
            if !response.status().is_success() {
                println!("😞 Something went wrong");
                println!("Status: {:?}", response.status());
                println!("Message: {:?}", response.text().unwrap());
                std::process::exit(EXIT_CODE_ERROR);
            }

            let json = serde_json::from_str::<ShioriResponseAddBookmark>(&response.text().unwrap())
                .unwrap();

            println!("🔖 Bookmark created");
            println!("URL:  {}", json.url);
            println!(
                "Tags: {}",
                json.tags
                    .iter()
                    .map(|tag| tag.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            std::process::exit(EXIT_CODE_SUCCESS);
        }
        Err(_error) => {
            println!("😞 Something went wrong");
            std::process::exit(EXIT_CODE_ERROR);
        }
    }
}

pub fn list(http_client: Client, persist: DiskPersist<LocalCache>) {
    if !validate::check_logged_in(&persist) {
        println!("🫤 You're not logged in");
        std::process::exit(EXIT_CODE_ERROR);
    }

    let local_cache = persist.read().unwrap().unwrap();
    let mut get_bookmarks_url = Url::parse(&local_cache.api_base_url).unwrap();
    get_bookmarks_url.set_path("api/bookmarks");
    let response = http_client
        .get(get_bookmarks_url)
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

            let mut bookmarks =
                serde_json::from_str::<ShioriResponseGetBookmarks>(&response.text().unwrap())
                    .unwrap()
                    .bookmarks;
            bookmarks.reverse();

            let last_bookmark = bookmarks.last();
            for bookmark in &bookmarks {
                println!("🔖 {}", bookmark.title);
                println!("URL:  {}", bookmark.url);
                println!(
                    "Tags: {}",
                    bookmark
                        .tags
                        .iter()
                        .map(|tag| tag.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                if bookmark.title != last_bookmark.unwrap().title {
                    println!();
                }
            }

            std::process::exit(EXIT_CODE_SUCCESS);
        }
        Err(_error) => {
            println!("😞 Something went wrong");
            std::process::exit(EXIT_CODE_ERROR);
        }
    }
}
