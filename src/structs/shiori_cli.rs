use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starts a wizard which guides you through the login
    Login {},
    /// Add a bookmark
    Add {
        /// Tags to apply to this bookmark, can be given multiple times
        #[arg(short, long)]
        tags: Vec<String>,

        url: String,
    },
    /// List existing tags
    GetTags {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCache {
    pub api_base_url: String,
    pub session_id: String,
    pub session_expires: String,
    pub username: String,
}
