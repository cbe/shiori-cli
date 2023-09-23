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

#[derive(Deserialize, Debug)]
pub struct ShioriResponseAddBookmark {
    pub url: String,
    pub title: String,
}
