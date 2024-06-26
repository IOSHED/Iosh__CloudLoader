use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api_manager::{google_drive::OAuthClient, interface::VerifyCloud},
    config::OAuthSecret,
    prelude::AuthResult,
    types::{Cloud, OAuthToken},
};

#[async_trait]
pub trait AuthCloud {
    async fn check(&self, cloud: Cloud, config: &OAuthSecret) -> AuthResult<OAuthToken>;
}

#[derive(Clone, Copy)]
pub struct NetAuthCloud;

unsafe impl Send for NetAuthCloud {}

#[async_trait]
impl AuthCloud for NetAuthCloud {
    async fn check(&self, cloud: Cloud, config: &OAuthSecret) -> AuthResult<OAuthToken> {
        match cloud {
            Cloud::GoogleDrive => {
                let client = Arc::new(OAuthClient::new(config.clone())?);
                client.clone().verify().await
            } // Cloud::YandexDisk => VerifyYandexDrive::new(config.clone())?.verify().await,
        }
    }
}
