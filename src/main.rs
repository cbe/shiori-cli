mod login;
mod structs;
mod validate;

use clap::Parser;
use disk_persist::DiskPersist;

use crate::login::login;
use crate::structs::shiori_cli::{Arguments, Commands, LocalCache};

fn main() {
    let arguments = Arguments::parse();
    let persist: DiskPersist<LocalCache> = DiskPersist::init("shiori-cli").unwrap();
    let http_client = reqwest::blocking::Client::new();

    match &arguments.command {
        Commands::Login {} => {
            login(http_client, persist);
        }
    }

}
