use async_trait::async_trait;

use crate::{config::OAuthSecret, AuthResult, OAuthToken};

#[async_trait]
pub trait Authorizer {
    async fn auth(self, config: OAuthSecret) -> AuthResult<OAuthToken>;
}
