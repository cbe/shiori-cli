use disk_persist::DiskPersist;
use requestty::Question;
use reqwest::blocking::Client;
use url::Url;

use crate::structs::{shiori_api::ShioriLogin, shiori_cli::LocalCache};
use crate::validate;

pub fn login(http_client: Client, persist: DiskPersist<LocalCache>) {
    let logged_in = validate::check_logged_in(&persist);

    if logged_in {
        println!("😎 Nothing to do, you seem to be logged in ")
    } else {
        let answers = ask_login_questions();
        let api_base_url = answers.api_base_url;
        let username = answers.username;
        let password = answers.password;

        let mut login_url = Url::parse(&api_base_url).unwrap();
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
            .unwrap()
            .text();

        match response {
            Ok(response) => match serde_json::from_str::<ShioriLogin>(response.as_str()) {
                Ok(json) => {
                    let data_to_persist = LocalCache {
                        api_base_url,
                        session_id: json.session,
                        session_expires: json.expires,
                        username: json.account.username,
                    };
                    persist.write(&data_to_persist).unwrap();

                    println!("🎉 Hello");
                }
                Err(_error) => {
                    println!("😞 Something went wrong");
                }
            },
            Err(_error) => {
                println!("😞 Something went wrong");
            }
        }
    }
}

struct LoginWizardAnswers {
    api_base_url: String,
    username: String,
    password: String,
}

fn ask_login_questions() -> LoginWizardAnswers {
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

    match requestty::prompt(questions) {
        Ok(answers) => {
            return LoginWizardAnswers {
                api_base_url: answers
                    .get("api_base_url")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_string(),
                username: answers
                    .get("username")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_string(),
                password: answers
                    .get("password")
                    .unwrap()
                    .as_string()
                    .unwrap()
                    .to_string(),
            }
        }
        Err(_error) => {
            panic!("Something went wrong while trying to login");
        }
    }
}
