mod login;
mod structs;

use crate::login::login;

fn main() {
    let http_client = reqwest::blocking::Client::new();

    login(http_client);

}
