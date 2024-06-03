/// Structure to hold an OAuth token.
///
/// # Fields
///
/// - `token`: A string slice representing the OAuth token.
#[derive(Clone)]
pub struct OAuthToken {
    pub token: String,
}
