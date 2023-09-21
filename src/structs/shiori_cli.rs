use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCache {
    pub api_base_url: String,
    pub session_id: String,
    pub session_expires: String,
    pub username: String,
}
