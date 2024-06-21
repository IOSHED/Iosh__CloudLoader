use async_trait::async_trait;

use crate::{CheckAliveResult, OAuthToken};

#[async_trait]
pub trait ApiCloud {
    async fn verify(&self) -> CheckAliveResult<OAuthToken>;
}
