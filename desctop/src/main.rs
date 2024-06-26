
use core_cloud_loader::{config::OAuthSecret, CheckerAlive, Cloud, NetAuthCloud};

mod infra;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth_config: OAuthSecret = OAuthSecret { 
        client_id: "-.apps.googleusercontent.com".to_string(), 
        auth_uri: "://../o//auth".to_string(), 
        token_uri: "://..com/".to_string(), 
        client_secret: "".to_string(), 
        redirect_uris: "http://localhost".to_string(),
    };

    let checker_alive = CheckerAlive::new(oauth_config);
    let res = checker_alive.is_cloud_alive(NetAuthCloud, Cloud::GoogleDrive).await;

    println!("{:#?}", res);

    Ok(())
}
