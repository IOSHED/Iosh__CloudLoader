#[derive(Debug, Clone)]
pub struct OAuthSecret {
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub client_secret: String,
    pub redirect_uris: String,
}
