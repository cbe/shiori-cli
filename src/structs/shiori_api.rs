use serde::{Deserialize, Serialize};

// Requests

#[derive(Deserialize, Serialize, Debug)]
pub struct ShioriRequestTag {
    pub name: String,
}

// Responses

#[derive(Deserialize, Debug)]
pub struct ShioriResponseLogin {
    pub session: String,
    pub expires: String,
    pub account: ShioriResponseUserAccount,
}

#[derive(Deserialize, Debug)]
pub struct ShioriResponseUserAccount {
    pub username: String,
}

pub type ShoiriResponseTag = ShioriRequestTag;

#[derive(Deserialize, Debug)]
pub struct ShioriResponseAddBookmark {
    pub url: String,
    pub title: String,
    pub tags: Vec<ShoiriResponseTag>,
}

#[derive(Deserialize, Debug)]
pub struct ShioriResponseGetTag {
    pub name: String,
}
