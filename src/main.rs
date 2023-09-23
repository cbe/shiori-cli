mod add;
mod login;
mod logout;
mod structs;
mod tags;
mod validate;

use clap::Parser;
use disk_persist::DiskPersist;

use crate::add::add;
use crate::login::login;
use crate::logout::logout;
use crate::structs::shiori_cli::{Arguments, Commands, LocalCache};

static EXIT_CODE_SUCCESS: i32 = 0;
static EXIT_CODE_ERROR: i32 = 1;

fn main() {
    let arguments = Arguments::parse();
    let persist: DiskPersist<LocalCache> = DiskPersist::init("shiori-cli").unwrap();
    let http_client = reqwest::blocking::Client::new();

    match &arguments.command {
        Commands::Login {} => {
            login(http_client, persist);
        }
        Commands::Logout {} => {
            logout(http_client, persist);
        }
        Commands::Add { tags, url } => {
            add(http_client, persist, tags.to_vec(), url.to_string());
        }
        Commands::GetTags {} => {
            tags::get_tags(http_client, persist);
        }
    }

}
