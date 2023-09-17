use requestty::Question;
use reqwest::blocking::Client;
use serde_json::json;
use url::Url;

fn main() {
    let http_client = reqwest::blocking::Client::new();

    login(http_client);
}

fn login(http_client: Client) {
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
                if name.trim().len() > 0 {
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

            let payload = json!({
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

            println!("body = {:?}", response);
        }
        Err(_error) => {}
    }
}
