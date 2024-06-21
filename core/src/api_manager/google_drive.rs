use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::AuthorizationCode;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use super::interface::ApiCloud;
use crate::{CheckAliveResult, OAuthToken};

pub struct ApiGoogleDrive;

#[async_trait]
impl ApiCloud for ApiGoogleDrive {
    async fn verify(&self) -> CheckAliveResult<OAuthToken> {
        // let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
        // let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

        // let client = BasicClient::new(
        //     ClientId::new(client_id),
        //     Some(ClientSecret::new(client_secret)),
        //     auth_url,
        //     Some(token_url),
        // )
        // .set_redirect_uri(RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string())?);

        // // Generate the authorization URL
        // let (auth_url, _csrf_token) = client
        //     .authorize_url(|| AuthorizationCode::new("".to_string()))
        //     .url();

        // println!("Browse to: {}", auth_url);

        // // Ask user for the authorization code
        // let mut auth_code = String::new();
        // println!("Enter the authorization code:");
        // std::io::stdin().read_line(&mut auth_code)?;

        // let token_result = client
        //     .exchange_code(AuthorizationCode::new(auth_code.trim().to_string()))
        //     .request_async(async_http_client)
        //     .await?;

        // Ok(token_result.access_token().secret().to_string())
        Ok(OAuthToken {
            token: "csodfk".to_string(),
        })
    }
}
