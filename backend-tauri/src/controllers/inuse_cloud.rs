use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    domain::unique_name::UniqueName,
    error::{ControllerResult, ErrorController},
    models::inuse_cloud::InuseCloud,
};

#[derive(Debug)]
pub struct InuseCloudController {
    inuse_clouds: Arc<Mutex<Vec<InuseCloud>>>,
}

impl Default for InuseCloudController {
    fn default() -> Self {
        Self {
            inuse_clouds: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl InuseCloudController {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_inuse_cloud(&self, new_inuse_clouds: Arc<Mutex<Vec<InuseCloud>>>) {
        let mut inuse_clouds = self.inuse_clouds.lock().await;
        let mut new_inuse_clouds = new_inuse_clouds.lock().await;
        inuse_clouds.append(&mut new_inuse_clouds);
    }

    pub async fn delete_cloud(&self, name_inuse_cloud: String) -> ControllerResult<()> {
        let mut inuse_clouds = self.inuse_clouds.lock().await;
        if let Some(index) = inuse_clouds
            .iter()
            .position(|cl| cl.name == name_inuse_cloud)
        {
            inuse_clouds.remove(index);
            return Ok(());
        }
        Err(ErrorController::ThisNameCloudNotFound(name_inuse_cloud))
    }

    pub async fn list_inuse_cloud(&self) -> ControllerResult<Vec<InuseCloud>> {
        Ok(self.inuse_clouds.lock().await.clone())
    }

    pub async fn rename(
        &self,
        from_name_inuse_cloud: String,
        to_name_inuse_cloud: String,
    ) -> ControllerResult<()> {
        let mut inuse_clouds = self.inuse_clouds.lock().await;

        if let Some(cloud) = inuse_clouds
            .iter_mut()
            .find(|cl| cl.name == from_name_inuse_cloud)
        {
            cloud.name = to_name_inuse_cloud;
            return Ok(());
        }
        Err(ErrorController::ThisNameCloudNotFound(
            from_name_inuse_cloud,
        ))
    }

    pub async fn generate_name_cloud(&self, name: Option<String>) -> String {
        let existing_names: Vec<String> = self
            .inuse_clouds
            .lock()
            .await
            .iter()
            .map(|cloud| cloud.name.clone())
            .collect();

        UniqueName::new(name, &existing_names).await.into()
    }
}
