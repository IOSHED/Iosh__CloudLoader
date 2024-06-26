use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use tokio::sync::oneshot;

use crate::config::OAuthSecret;
use crate::{AuthError, AuthResult, OAuthToken};

pub struct OAuthClient {
    client: BasicClient,
    token_sender: Mutex<Option<oneshot::Sender<OAuthToken>>>,
}

impl OAuthClient {
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
            client: client,
            token_sender: Mutex::new(None),
        })
    }

    fn authorize_url(&self) -> (String, CsrfToken) {
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/drive.file".to_string(),
            ))
            .url();

        (auth_url.to_string(), csrf_token)
    }

    async fn handle_callback(&self, query: HashMap<String, String>) -> AuthResult<OAuthToken> {
        let auth_code = query
            .get("code")
            .ok_or(AuthError::FailedGetTockenResult)?
            .to_string();

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(auth_code.trim().to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|_| AuthError::FailedGetTockenResult)?;

        Ok(OAuthToken {
            refresh_token: token_result.refresh_token().map(|r| r.secret().clone()),
            access_token: token_result.access_token().secret().clone(),
        })
    }

    async fn callback(
        query: web::Query<HashMap<String, String>>,
        oauth_client: web::Data<Arc<OAuthClient>>,
    ) -> impl Responder {
        match oauth_client.handle_callback(query.into_inner()).await {
            Ok(token) => {
                if let Some(sender) = oauth_client.token_sender.lock().unwrap().take() {
                    let _ = sender.send(token);
                }
                HttpResponse::Ok().body("Authorization successful! You can close this window now.")
            }
            Err(err) => {
                eprintln!("Error handling callback: {:?}", err);
                HttpResponse::InternalServerError().body("Authorization failed.")
            }
        }
    }

    pub async fn verify(self: Arc<Self>) -> AuthResult<OAuthToken> {
        let (sender, receiver) = oneshot::channel();
        *self.token_sender.lock().unwrap() = Some(sender);
   
        let (auth_url, _csrf_token) = self.authorize_url();
        println!(
            "Please open the following URL in your browser: {}",
            auth_url
        );

        let oauth_client = web::Data::new(self);

        let http_server = HttpServer::new(move || {
            App::new()
                .app_data(oauth_client.clone())
                .route("/callback", web::get().to(OAuthClient::callback))
        })
        .bind("127.0.0.1:3030")
        .map_err(|_| AuthError::FailedBindServer)?;

        tokio::spawn(http_server.run());
        
        match receiver.await {
            Ok(token) => Ok(token),
            Err(_) => Err(AuthError::FailedReceiveToken),
        }
    }
}


// #[async_trait]
// impl VerifyCloud for OAuthClient {
//     async fn verify(self: Arc<Self>) -> AuthResult<OAuthToken> {
//         let (sender, receiver) = oneshot::channel();
//         *self.token_sender.lock().unwrap() = Some(sender);

//         let (auth_url, _csrf_token) = self.authorize_url();
//         println!("Please open the following URL in your browser: {}", auth_url);

//         // Run the server in a new task to avoid blocking the current thread
//         let server_future = tokio::spawn(self.clone().run_server());

//         match receiver.await {
//             Ok(token) => Ok(token),
//             Err(_) => Err(AuthError::FailedReceiveToken),
//         }?;

//         // Ensure the server future completes without errors
//         server_future.await.map_err(|_| AuthError::JoinTokioTaskFailed)?
//     }
// }
