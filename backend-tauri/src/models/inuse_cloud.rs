use core_cloud_loader::{Cloud, OAuthToken};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct InuseCloud {
    pub name: String,
    pub cloud: Cloud,
    pub token: Option<OAuthToken>,
    pub inuse_memory: Option<f32>,
    pub all_memory: Option<f32>,
}

impl InuseCloud {
    pub fn new(name: String, cloud: Cloud) -> Self {
        Self {
            cloud,
            name,
            token: None,
            inuse_memory: None,
            all_memory: None,
        }
    }
}
