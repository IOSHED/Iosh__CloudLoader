pub mod controllers;
pub mod domain;
pub mod error;

mod commands;
mod models;

use std::sync::Arc;

use controllers::{InuseCloudController, RepositoryCloudController};
use tokio::sync::Mutex;

use crate::commands::{
    add_cloud_to_repository, create_new_repository_cloud, delete_cloud, delete_repository_cloud,
    get_reading_docs, list_all_inuse_cloud, list_all_repository_cloud, move_cloud,
    rename_inuse_cloud, rename_repository_cloud,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let inuse_cloud_controller = Arc::new(Mutex::new(InuseCloudController::new()));
    let repository_cloud_controller =
        RepositoryCloudController::new(inuse_cloud_controller.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(repository_cloud_controller)
        .manage(inuse_cloud_controller.clone())
        .invoke_handler(tauri::generate_handler![
            create_new_repository_cloud,
            list_all_repository_cloud,
            list_all_inuse_cloud,
            get_reading_docs,
            delete_repository_cloud,
            delete_cloud,
            rename_repository_cloud,
            rename_inuse_cloud,
            add_cloud_to_repository,
            move_cloud,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
