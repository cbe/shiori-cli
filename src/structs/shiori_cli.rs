use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Login {},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCache {
    pub api_base_url: String,
    pub session_id: String,
    pub session_expires: String,
    pub username: String,
}
