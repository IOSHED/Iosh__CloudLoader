#![allow(clippy::all)]

use core_cloud_loader::{api, config::OAuthSecret, AuthUserCloud, Cloud, NetAuthCloud};

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
    let token = res.unwrap().access_token;

    println!("{:#?}", token.clone());

    let err = api::list_files(&token).await;

    println!("{:#?}", err);

    let err = api::upload_file(&token, "file.txt", "text/plain", None).await;

    println!("{:#?}", err);

    let err = api::list_files(&token).await;

    println!("{:#?}", err);

    let err = api::download_file(&token, "1nizWPaZHOuQaud4xgu66SJB0ERdKnlXp", "f.txt").await;

    println!("{:#?}", err);

    Ok(())
}
