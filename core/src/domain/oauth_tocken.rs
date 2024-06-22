/// Structure to hold an OAuth token.
///
/// # Fields
///
/// - `token`: A string slice representing the OAuth token.
#[derive(Debug, Clone)]
pub struct OAuthToken {
    pub refresh_token: Option<String>,
    pub access_token: String,
}
