use async_trait::async_trait;

use crate::{
    api_manager::{google_drive::VerifyGoogleDrive, interface::VerifyCloud}, config::OAuthSecret, types::{Cloud, OAuthToken}, prelude::AuthResult
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
            Cloud::GoogleDrive => VerifyGoogleDrive::new(config.clone())?.verify().await,
            // Cloud::YandexDisk => VerifyYandexDrive::new(config.clone())?.verify().await,
        }
    }
}
