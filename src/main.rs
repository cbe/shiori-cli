mod login;
mod structs;

use clap::Parser;
use disk_persist::DiskPersist;
use structs::shiori_cli::{Arguments, Commands, LocalCache};

use crate::login::login;

fn main() {
    let arguments = Arguments::parse();
    let persist: DiskPersist<LocalCache> = DiskPersist::init("shiori-cli").unwrap();
    let http_client = reqwest::blocking::Client::new();

    match &arguments.command {
        Some(Commands::Login {}) => {
            login(persist, http_client);
        }
        None => {}
    }

}
