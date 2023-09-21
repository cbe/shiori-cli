use disk_persist::DiskPersist;
use requestty::Question;
use reqwest::blocking::Client;
use url::Url;

use crate::structs::{shiori_api::ShioriLogin, shiori_cli::LocalCache};

pub fn login(persist: DiskPersist<LocalCache>, http_client: Client) {
    let local_cache = persist.read().unwrap();

    match local_cache {
        Some(_local_cache) => {
            println!("Nothing to do, you seem to be logged in");
        }
        None => {
            let questions = vec![
                Question::input("api_base_url")
                    .message("Where's your shiori instance located at")
                    .validate(|api_base_url, _previous_answers| {
                        if Url::parse(api_base_url).is_ok() {
                            Ok(())
                        } else {
                            Err("Please enter a valid URL".to_owned())
                        }
                    })
                    .build(),
                Question::input("username")
                    .message("What's your username")
                    .validate(|name, _previous_answers| {
                        if !name.trim().is_empty() {
                            Ok(())
                        } else {
                            Err("Please enter your username".to_owned())
                        }
                    })
                    .build(),
                Question::password("password")
                    .message("What's your password")
                    .build(),
            ];

            let answers = requestty::prompt(questions);

            match answers {
                Ok(answers) => {
                    let api_base_url = answers.get("api_base_url").unwrap().as_string();
                    let username = answers.get("username").unwrap().as_string();
                    let password = answers.get("password").unwrap().as_string();

                    let mut login_url = Url::parse(api_base_url.unwrap()).unwrap();
                    login_url.set_path("api/login");

                    let payload = serde_json::json!({
                        "username": username,
                        "password": password,
                        "remember": true,
                    });

                    let response = http_client
                        .post(login_url)
                        .json(&payload)
                        .send()
                        .expect("HTTP Response")
                        .text();

                    let login: ShioriLogin =
                        serde_json::from_str(response.unwrap().as_str()).unwrap();
                    let data_to_persist = LocalCache {
                        api_base_url: api_base_url.unwrap().to_string(),
                        session_id: login.session,
                        session_expires: login.expires,
                        username: login.account.username,
                    };

                    persist.write(&data_to_persist).unwrap();
                    println!("Welcome 🤗");
                }
                Err(_error) => {}
            }
        }
    }
}
