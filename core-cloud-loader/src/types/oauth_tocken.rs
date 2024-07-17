use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthToken {
    pub refresh_token: Option<String>,
    pub access_token: String,
}
