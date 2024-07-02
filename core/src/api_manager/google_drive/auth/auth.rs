use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};

use async_trait::async_trait;

use crate::{api_manager::interface::Authorizer, config::OAuthSecret, AuthError, AuthResult, OAuthToken};

use super::{client::OAuthClient, manager::OAuthManager};



pub struct GoogleDriveAuth;

#[async_trait]
impl Authorizer for GoogleDriveAuth {
    async fn auth(self, config: OAuthSecret) -> AuthResult<OAuthToken> {
        let oauth_veryfer = OAuthManager::new(config)?;

        let (sender, receiver) = tokio::sync::oneshot::channel();
        let cli = OAuthClient::new(oauth_veryfer.get_basic_client(), sender)?;
        tokio::spawn(async move {
            let arc_cli = Arc::new(cli);
            let addrs = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3001);
            let _ = arc_cli.run(addrs, "/callback".to_string()).await;
        });

        oauth_veryfer.generate_and_browse_auth_url().await?;

        if let Ok(res) = receiver.await {
            return Ok(res)
        }

        Err(AuthError::JoinTokioTaskFailed)
    }
}