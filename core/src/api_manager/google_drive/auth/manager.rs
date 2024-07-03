use std::sync::Arc;

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use reqwest::Url;

use crate::config::OAuthSecret;
use crate::{AuthError, AuthResult};

pub struct OAuthManager {
    client: Arc<BasicClient>,
}

impl OAuthManager {
    pub fn new(config: OAuthSecret) -> AuthResult<Self> {
        let auth_uri = AuthUrl::new(config.auth_uri).map_err(|_| AuthError::FailedGotAuthUrl)?;
        let token_uri =
            TokenUrl::new(config.token_uri).map_err(|_| AuthError::FailedGotTockenUrl)?;
        let redirect_uris =
            RedirectUrl::new(config.redirect_uris).map_err(|_| AuthError::FailedGotRedirectUrl)?;

        let client_id = ClientId::new(config.client_id);
        let client_secret = ClientSecret::new(config.client_secret);

        let client = BasicClient::new(client_id, Some(client_secret), auth_uri, Some(token_uri))
            .set_redirect_uri(redirect_uris);

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn generate_and_browse_auth_url(&self) -> AuthResult<()> {
        let (auth_url, _csrf_token) = self.authorize_url();
        webbrowser::open(&auth_url.to_string()).map_err(|_| AuthError::FailedOpenBrowser)?;
        Ok(())
    }

    pub fn get_basic_client(&self) -> Arc<BasicClient> {
        self.client.clone()
    }

    fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive.file".to_string(),
            ))
            .url()
    }
}
