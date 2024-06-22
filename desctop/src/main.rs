use core_cloud_loader::{is_cloud_alive, Cloud};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = is_cloud_alive(Cloud::GoogleDrive).await;
    println!("{:#?}", r);

    Ok(())
}
