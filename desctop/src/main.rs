
use core_cloud_loader::{config::OAuthSecret, AuthUserCloud, Cloud, NetAuthCloud};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let oauth_config: OAuthSecret = OAuthSecret { 
        client_id: "-.apps.googleusercontent.com".to_string(), 
        auth_uri: "://../o//auth".to_string(), 
        token_uri: "://..com/".to_string(), 
        client_secret: "".to_string(), 
        redirect_uris: "http://localhost".to_string(),
    };

    let auther = AuthUserCloud::new(oauth_config);
    let res = auther.is_cloud(NetAuthCloud, Cloud::GoogleDrive).await;

    println!("{:#?}", res);

    Ok(())
}
