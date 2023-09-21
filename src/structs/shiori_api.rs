use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ShioriLogin {
    pub session: String,
    pub expires: String,
    pub account: ShioriUserAccount,
}

#[derive(Deserialize, Debug)]
pub struct ShioriUserAccount {
    pub username: String,
}
