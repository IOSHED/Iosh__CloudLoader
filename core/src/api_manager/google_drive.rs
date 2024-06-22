use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};

use super::interface::ApiCloud;
use crate::{CheckAliveError, CheckAliveResult, OAuthToken};

pub struct ApiGoogleDrive;

#[async_trait]
impl ApiCloud for ApiGoogleDrive {
    async fn verify(&self) -> CheckAliveResult<OAuthToken> {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
            .map_err(|_| CheckAliveError::FailedGotAuthUrl)?;

        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .map_err(|_| CheckAliveError::FailedGotTockenUrl)?;

        let redirect_url = RedirectUrl::new("http://localhost".to_string())
            .map_err(|_| CheckAliveError::FailedGotRedirectUrl)?;

        let client_id = ClientId::new(
            "392071929150-apcm9qgbgmr4hqlp5l2dedp316kp0nep.apps.googleusercontent.com".to_string(),
        );

        let client_secret = ClientSecret::new("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".to_string());

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        // Generate the full authorization URL.
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive.file".to_string(),
            ))
            // .add_scope(Scope::new("write".to_string()))
            // Set the PKCE code challenge.
            // .set_pkce_challenge(pkce_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        println!("Browse to: {}", auth_url);

        let mut auth_code = String::new();
        println!("Enter the authorization code:");
        let _ = std::io::stdin().read_line(&mut auth_code);

        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_state`.

        // Now you can trade it for an access token.
        let token_result = client
            .exchange_code(AuthorizationCode::new(auth_code.trim().to_string()))
            // Set the PKCE code verifier.
            // .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|_| CheckAliveError::FailedGetTockenResult)?;

        // Ok(token_result.access_token().secret().to_string())
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
