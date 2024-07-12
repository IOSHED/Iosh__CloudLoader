use std::error::Error;

use reqwest::multipart;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

/// Structure to represent file metadata
#[derive(Debug, Deserialize)]
pub struct FileMetadata {
    name: String,
    id: String,
}

/// Structure to represent a list of files
#[derive(Debug, Deserialize)]
pub struct FileList {
    files: Vec<FileMetadata>,
}

/// Lists files in Google Drive
pub async fn list_files(token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client
        .get("https://www.googleapis.com/drive/v3/files")
        .bearer_auth(token)
        .send()
        .await?;

    let file_list: FileList = res.json().await?;
    // TODO: delete
    let _ = (
        file_list.files[0].id.clone(),
        file_list.files[0].name.clone(),
    );
    println!("{:#?}", file_list.files);
    Ok(())
}

/// Downloads a file from Google Drive
pub async fn download_file(
    token: &str,
    file_id: &str,
    destination: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media",
        file_id
    );

    let response = client.get(&url).bearer_auth(token).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_text,
        )));
    }

    let mut file = File::create(destination).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;

    println!("File downloaded successfully");
    Ok(())
}

/// Uploads a file to Google Drive
pub async fn upload_file(
    token: &str,
    file_path: &str,
    mime_type: &str,
    folder_id: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Read file content
    let mut file = File::open(file_path).await?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).await?;

    // Create multipart form
    let file_part = multipart::Part::bytes(file_content)
        .mime_str(mime_type)?
        .file_name(file_path.to_string());

    // Add metadata
    let mut metadata = json!({
        "name": file_path,
        "mimeType": mime_type,
    });

    if let Some(folder_id) = folder_id {
        metadata["parents"] = json!([folder_id]);
    }

    // Create metadata part
    let metadata_part =
        multipart::Part::text(metadata.to_string()).mime_str("application/json; charset=UTF-8")?;

    // Create the form with metadata first
    let form = multipart::Form::new()
        .part("metadata", metadata_part)
        .part("file", file_part);

    // Make request
    let response = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?;

    println!("{:#?}", response);

    Ok(())
}
