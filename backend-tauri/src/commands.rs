use core_cloud_loader::Cloud;
use tauri::State;

use crate::{
    controllers::{InuseCloudController, RepositoryCloudController},
    error::ControllerResult,
    models::{inuse_cloud::InuseCloud, repository_cloud::RepositoryCloud},
};

#[tauri::command]
pub async fn create_new_repository_cloud(
    controller: State<'_, RepositoryCloudController>,
    name_repository: Option<String>,
    clouds: Vec<(Option<String>, String)>,
) -> ControllerResult<Vec<RepositoryCloud>> {
    let clouds: Vec<(Option<String>, Cloud)> = clouds.into_iter().map(|(name, cloud)| (name, cloud.into())).collect();
    controller
        .create_repository_cloud(name_repository, clouds)
        .await?;
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn list_all_repository_cloud(
    controller: State<'_, RepositoryCloudController>,
) -> ControllerResult<Vec<RepositoryCloud>> {
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn delete_repository_cloud(
    controller: State<'_, RepositoryCloudController>,
    name_repository: String,
) -> ControllerResult<Vec<RepositoryCloud>> {
    controller.delete_repository_cloud(name_repository).await?;
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn delete_cloud(
    controller: State<'_, InuseCloudController>,
    name_inuse_cloud: String,
) -> ControllerResult<Vec<InuseCloud>> {
    controller.delete_cloud(name_inuse_cloud).await?;
    controller.list_inuse_cloud().await
}

#[tauri::command]
pub async fn add_cloud_to_repository(
    controller: State<'_, RepositoryCloudController>,
    name_repository: String,
    name_inuse_cloud: Option<String>,
    cloud: String,
) -> ControllerResult<Vec<RepositoryCloud>> {
    controller
        .add_new_cloud(name_repository, name_inuse_cloud, cloud.into())
        .await?;
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn move_cloud(
    controller: State<'_, RepositoryCloudController>,
    name_inuse_cloud: String,
    from_name_repository: String,
    to_name_repository: String,
) -> ControllerResult<Vec<RepositoryCloud>> {
    controller
        .move_cloud(name_inuse_cloud, from_name_repository, to_name_repository)
        .await?;
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn rename_repository_cloud(
    controller: State<'_, RepositoryCloudController>,
    from_name_repository: String,
    to_name_repository: String,
) -> ControllerResult<Vec<RepositoryCloud>> {
    controller
        .rename(from_name_repository, to_name_repository)
        .await?;
    controller.list_repository_cloud().await
}

#[tauri::command]
pub async fn rename_inuse_cloud(
    controller: State<'_, InuseCloudController>,
    from_name_inuse_cloud: String,
    to_name_inuse_cloud: String,
) -> ControllerResult<Vec<InuseCloud>> {
    controller
        .rename(from_name_inuse_cloud, to_name_inuse_cloud)
        .await?;
    controller.list_inuse_cloud().await
}

#[tauri::command]
pub async fn list_all_inuse_cloud(
    controller: State<'_, InuseCloudController>,
) -> ControllerResult<Vec<InuseCloud>> {
    controller.list_inuse_cloud().await
}

#[tauri::command]
pub fn get_reading_docs() -> String {
    "Здесь должна была быть документция.".into()
}
