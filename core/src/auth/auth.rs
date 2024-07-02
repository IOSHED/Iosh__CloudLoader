
use async_trait::async_trait;

use crate::{
    api_manager::{google_drive::auth::auth::GoogleDriveAuth, interface::Authorizer}, config::OAuthSecret, prelude::AuthResult, types::{Cloud, OAuthToken}, AuthError
};

#[async_trait]
pub trait AuthCloud {
    async fn auth(&self, cloud: Cloud, config: OAuthSecret) -> AuthResult<OAuthToken>;
}

#[derive(Clone, Copy)]
pub struct NetAuthCloud;

unsafe impl Send for NetAuthCloud {}

#[async_trait]
impl AuthCloud for NetAuthCloud {
    async fn auth(&self, cloud: Cloud, config: OAuthSecret) -> AuthResult<OAuthToken> {
        match cloud {
            Cloud::GoogleDrive => GoogleDriveAuth.auth(config).await,
        }
    }
}
