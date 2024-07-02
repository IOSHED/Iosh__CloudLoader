use futures::future;

use super::AuthCloud;
use crate::{
    config::OAuthSecret,
    prelude::AuthResult,
    types::{Cloud, OAuthToken},
    AuthError,
};

#[derive(Debug, Clone)]
pub struct AuthUserCloud {
    config: OAuthSecret,
}

impl AuthUserCloud {
    pub fn new(config: OAuthSecret) -> Self {
        Self { config }
    }

    pub async fn are_all_clouds<C>(
        self,
        checker: C,
        clouds_with_auth: &[Cloud],
    ) -> Vec<AuthResult<OAuthToken>>
    where
        C: AuthCloud + Copy + Send + 'static,
    {
        let tasks: Vec<_> = clouds_with_auth
            .iter()
            .cloned()
            .map(|cloud| {
                tokio::task::spawn({
                    let value = self.clone();
                    async move { value.is_cloud(checker, cloud).await }
                })
            })
            .collect();

        let results = future::join_all(tasks).await;
        results
            .into_iter()
            .map(|res| res.unwrap_or(Err(AuthError::JoinTokioTaskFailed)))
            .collect()
    }

    pub async fn is_cloud<C>(self, checker: C, cloud: Cloud) -> AuthResult<OAuthToken>
    where
        C: AuthCloud,
    {
        checker.auth(cloud, self.config).await
    }
}
