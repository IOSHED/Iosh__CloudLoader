use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use async_trait::async_trait;

use super::{client::OAuthClient, manager::OAuthManager};
use crate::{
    api_manager::interface::Authorizer, config::OAuthSecret, AuthError, AuthResult, OAuthToken,
};

pub struct GoogleDriveAuth;

#[async_trait]
impl Authorizer for GoogleDriveAuth {
    async fn auth(self, config: OAuthSecret) -> AuthResult<OAuthToken> {
        let oauth_manager = OAuthManager::new(config)?;

        let (sender, receiver) = tokio::sync::oneshot::channel();
        let client = OAuthClient::new(oauth_manager.get_basic_client(), sender)?;
        let client_arc = Arc::new(client);

        tokio::spawn(run_server(client_arc));

        oauth_manager.generate_and_browse_auth_url().await?;

        receiver.await.map_err(|_| AuthError::JoinTokioTaskFailed)
    }
}

async fn run_server(client: Arc<OAuthClient>) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3001);
    let _ = client.run(addr, "/callback".to_string()).await;
}
