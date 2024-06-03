use std::path::Path;

use tokio::fs::File;

use crate::domain::{AddressFile, Cloud};

/// Structure to thandle loading data in cloud services.
///
/// # Fields
///
/// - `use_clouds`: A slice of `Cloud` enums representing the cloud services to be used.
pub struct LoaderInClouds<'a> {
    use_clouds: &'a [Cloud],
}

impl<'a> LoaderInClouds<'a> {
    /// Creates a new instance of `LoaderInClouds`.
    ///
    /// # Arguments
    ///
    /// - `use_clouds`: A slice of `Cloud` enums representing the cloud services to be used.
    ///
    /// # Returns
    ///
    /// - `LoaderInClouds`: A new instance of `LoaderInClouds`.
    pub fn new(use_clouds: &'a [Cloud]) -> Self {
        Self { use_clouds }
    }

    /// Uploads files to the specified cloud storage. At the same time, files can be placed in different clouds.
    ///
    /// # Arguments
    /// - `loading_files`: All files that will be uploaded.
    ///
    /// # Returns
    /// - `Vec<AddressFile>`: All addresses of successfully uploaded files.
    pub async fn upload_files<'p>(self, loading_files: &[File]) -> Vec<AddressFile<'p>> {
        // TODO: Implement the actual logic to load data in clouds.

        vec![AddressFile {
            cloud: Cloud::GoogleDrive,
            path: Path::new(""),
        }]
    }
}
