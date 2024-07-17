use std::sync::Arc;

use core_cloud_loader::Cloud;
use futures::future::join_all;
use tokio::sync::Mutex;

use super::InuseCloudController;
use crate::{
    domain::unique_name::UniqueName,
    error::{ControllerResult, ErrorController},
    models::{inuse_cloud::InuseCloud, repository_cloud::RepositoryCloud},
};

#[derive(Debug)]
pub struct RepositoryCloudController {
    repositories: Arc<Mutex<Vec<RepositoryCloud>>>,
    inuse_cloud_controller: Arc<Mutex<InuseCloudController>>,
}

impl RepositoryCloudController {
    pub fn new(inuse_cloud_controller: Arc<Mutex<InuseCloudController>>) -> Self {
        Self {
            repositories: Arc::new(Mutex::new(vec![])),
            inuse_cloud_controller,
        }
    }

    pub async fn create_repository_cloud(
        &self,
        name_repository: Option<String>,
        clouds: Vec<(Option<String>, Cloud)>,
    ) -> ControllerResult<()> {
        let mut repositories = self.repositories.lock().await;
        let inuse_cloud_controller = self.inuse_cloud_controller.lock().await;

        let inuse_clouds = join_all(clouds.into_iter().map(|(name, cloud)| async {
            InuseCloud::new(self.generate_name_cloud(name).await, cloud)
        }))
        .await;

        let arc_inuse_clouds = Arc::new(Mutex::new(inuse_clouds));

        let new_repository_cloud = RepositoryCloud::new(
            self.generate_name_repository(name_repository).await,
            arc_inuse_clouds.clone(),
        );
        repositories.push(new_repository_cloud);

        inuse_cloud_controller
            .add_inuse_cloud(arc_inuse_clouds.clone())
            .await;

        Ok(())
    }

    pub async fn list_repository_cloud(&self) -> ControllerResult<Vec<RepositoryCloud>> {
        Ok(self.repositories.lock().await.clone())
    }

    pub async fn delete_repository_cloud(&self, name_repository: String) -> ControllerResult<()> {
        let mut repositories = self.repositories.lock().await;
        let index = repositories
            .iter()
            .position(|repo| repo.name == name_repository)
            .ok_or(ErrorController::ThisNameRepositoryNotFound(name_repository))?;
        repositories.remove(index);
        Ok(())
    }

    pub async fn add_new_cloud(
        &self,
        name_repository: String,
        name_inuse_cloud: Option<String>,
        cloud: Cloud,
    ) -> ControllerResult<()> {
        let mut repositories = self.repositories.lock().await;

        let repo = repositories
            .iter_mut()
            .find(|repo| repo.name == name_repository)
            .ok_or(ErrorController::ThisNameRepositoryNotFound(
                name_repository.clone(),
            ))?;

        let new_cloud = InuseCloud::new(self.generate_name_cloud(name_inuse_cloud).await, cloud);
        repo.inuse_clouds.lock().await.push(new_cloud);

        Ok(())
    }

    pub async fn move_cloud(
        &self,
        name_inuse_cloud: String,
        from_name_repository: String,
        to_name_repository: String,
    ) -> ControllerResult<()> {
        let mut repositories = self.repositories.lock().await;

        let move_inuse_cloud = {
            let from_repo = repositories
                .iter()
                .find(|repo| repo.name == from_name_repository)
                .ok_or(ErrorController::ThisNameRepositoryNotFound(
                    from_name_repository.clone(),
                ))?;

            from_repo
                .inuse_clouds
                .lock()
                .await
                .iter()
                .find(|cl| cl.name == name_inuse_cloud)
                .ok_or(ErrorController::ThisNameCloudNotFound(
                    name_inuse_cloud.clone(),
                ))?
                .clone()
        };

        let to_repo = repositories
            .iter_mut()
            .find(|repo| repo.name == to_name_repository)
            .ok_or(ErrorController::ThisNameRepositoryNotFound(
                to_name_repository.clone(),
            ))?;

        let inuse_cloud_controller = self.inuse_cloud_controller.lock().await;
        inuse_cloud_controller
            .delete_cloud(name_inuse_cloud)
            .await?;

        to_repo.inuse_clouds.lock().await.push(move_inuse_cloud);

        Ok(())
    }

    pub async fn rename(
        &self,
        from_name_repository: String,
        to_name_repository: String,
    ) -> ControllerResult<()> {
        let mut repositories = self.repositories.lock().await;

        let repo = repositories
            .iter_mut()
            .find(|repo| repo.name == from_name_repository)
            .ok_or(ErrorController::ThisNameRepositoryNotFound(
                from_name_repository.clone(),
            ))?;

        repo.name = to_name_repository;

        Ok(())
    }

    async fn generate_name_cloud(&self, name: Option<String>) -> String {
        self.inuse_cloud_controller
            .lock()
            .await
            .generate_name_cloud(name)
            .await
    }

    async fn generate_name_repository(&self, name: Option<String>) -> String {
        let existing_names: Vec<String> = self
            .repositories
            .lock()
            .await
            .iter()
            .map(|repo| repo.name.clone())
            .collect();

        UniqueName::new(name, &existing_names).await.into()
    }
}
