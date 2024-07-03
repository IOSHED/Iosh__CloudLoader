use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use tokio::sync::oneshot;

use crate::{AuthError, AuthResult, OAuthToken};

pub struct OAuthClient {
    client: Arc<BasicClient>,
    token_sender: Arc<Mutex<Option<oneshot::Sender<OAuthToken>>>>,
}

impl OAuthClient {
    pub fn new(
        client: Arc<BasicClient>,
        token_sender: oneshot::Sender<OAuthToken>,
    ) -> AuthResult<Self> {
        Ok(Self {
            client,
            token_sender: Arc::new(Mutex::new(Some(token_sender))),
        })
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
                let mut sender_guard = oauth_client.token_sender.lock().unwrap();
                if let Some(sender) = sender_guard.take() {
                    let _ = sender.send(token);
                }
                HttpResponse::Ok().body("Authorization successful! You can close this window now.")
            }
            Err(_) => HttpResponse::InternalServerError().body("Authorization failed."),
        }
    }

    pub async fn run<A>(self: Arc<Self>, addrs: A, path: String) -> AuthResult<()>
    where
        A: std::net::ToSocketAddrs,
    {
        let oauth_client = web::Data::new(self.clone());
        let http_server = HttpServer::new(move || {
            App::new()
                .app_data(oauth_client.clone())
                .route(&path, web::get().to(OAuthClient::callback))
        })
        .bind(addrs)
        .map_err(|_| AuthError::FailedBindServer)?;

        http_server
            .run()
            .await
            .map_err(|_| AuthError::FailedBindServer)?;

        Ok(())
    }
}
