use async_trait::async_trait;

use super::interface::ApiCloud;
use crate::{CheckAliveResult, OAuthToken};

pub struct ApiYandexDisk;

#[async_trait]
impl ApiCloud for ApiYandexDisk {
    async fn verify(&self) -> CheckAliveResult<OAuthToken> {
        Ok(OAuthToken {
            token: "csodfk".to_string(),
        })
    }
}
