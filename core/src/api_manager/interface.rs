use async_trait::async_trait;

use crate::{AuthResult, OAuthToken};

#[async_trait]
pub trait VerifyCloud {
    async fn verify(self) -> AuthResult<OAuthToken>;
}

// #[async_trait]
// pub trait ApiCloud {
//     async fn upload_file(&self, file_path: &str, mime_type: &str, folder_id: Option<&str>) -> ApiResult<()>;
//     async fn load_file(&self, file_id: &str, destination: &str) -> ApiResult<()>;
//     async fn list_files(&self) -> ApiResult<()>;
// }
