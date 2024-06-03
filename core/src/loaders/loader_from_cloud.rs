use std::{collections::HashMap, str::Bytes};

use tokio::fs::File;

use crate::domain::AddressFile;

/// Structure to thandle loading data from cloud services.
///
/// # Fields
///
/// - `hash_got_file`: Hash where you can get the contents of the `AddressFile`.
pub struct LoaderFromCloud<'a> {
    hash_got_file: HashMap<AddressFile<'a>, Bytes<'a>>,
}

impl<'a> LoaderFromCloud<'a> {
    /// Creates a new instance of `LoaderFromClouds`.
    ///
    /// # Returns
    ///
    /// - `LoaderFromClouds`: A new instance of `LoaderFromClouds`.
    pub fn new() -> Self {
        Self {
            hash_got_file: HashMap::new(),
        }
    }
    /// Loads files to the specified cloud storage. At the same time, files can be placed in different clouds.
    ///
    /// # Arguments
    /// - `address_files`: The slice of address of the file where its contents will be received.
    ///
    /// # Returns
    /// - `Vec<File>`: All files that were received successfully.
    pub async fn get_data<'p>(self, address_files: &[AddressFile<'p>]) -> Vec<File> {
        // TODO: Implement the actual logic to load data from clouds.

        let file = File::open("too.txt").await.unwrap();
        vec![file]
    }
}
