use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthorizationCode, CsrfToken, Scope,
    TokenResponse, AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl
};

use crate::api_manager::interface::VerifyCloud;
use crate::config::OAuthSecret;
use crate::{AuthError, AuthResult, OAuthToken};

pub struct VerifyGoogleDrive {
    client_id: ClientId,
    auth_uri: AuthUrl,
    token_uri: TokenUrl,
    client_secret: ClientSecret,
    redirect_uris: RedirectUrl,
}

impl VerifyGoogleDrive {
    pub fn new(config: OAuthSecret) -> AuthResult<Self> {
        let auth_uri = AuthUrl::new(config.auth_uri)
            .map_err(|_| AuthError::FailedGotAuthUrl)?;

        let token_uri = TokenUrl::new(config.token_uri)
            .map_err(|_| AuthError::FailedGotTockenUrl)?;

        let redirect_uris = RedirectUrl::new(config.redirect_uris)
            .map_err(|_| AuthError::FailedGotRedirectUrl)?;

        let client_id = ClientId::new(config.client_id);

        let client_secret = ClientSecret::new(config.client_secret);
        
        Ok(Self {
            client_id,
            auth_uri,
            token_uri,
            client_secret,
            redirect_uris,
        })
    }
}

#[async_trait]
impl VerifyCloud for VerifyGoogleDrive {
    async fn verify(self) -> AuthResult<OAuthToken> {

        let client = BasicClient::new(
            self.client_id,
            Some(self.client_secret), 
            self.auth_uri, 
            Some(self.token_uri)
        )
        .set_redirect_uri(self.redirect_uris);

        // Generate the full authorization URL.
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive.file".to_string(),
            ))
            .url();

        println!("Browse to: {}", auth_url);

        let mut auth_code = String::new();
        println!("Enter the authorization code:");
        let _ = std::io::stdin().read_line(&mut auth_code);

        let token_result = client
            .exchange_code(AuthorizationCode::new(auth_code.trim().to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|_| AuthError::FailedGetTockenResult)?;

        let refresh_token: Option<String> = match token_result.refresh_token() {
            Some(res) => Some(res.secret().clone()),
            None => None,
        };

        Ok(OAuthToken {
            refresh_token: refresh_token,
            access_token: token_result.access_token().secret().clone(),
        })
    }
}
