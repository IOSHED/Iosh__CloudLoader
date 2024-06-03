//! This module provides an enumeration for different cloud storage services, structures for OAuth tokens and file addresses, and functions to check cloud service availability and load data from clouds.
//!
//! # Enums
//!
//! - `Cloud`: Represents different cloud storage services.
//!
//! # Structs
//!
//! - `OAuthToken`: A structure to hold an OAuth token as a string slice.
//! - `AddressFile`: A structure to associate a cloud service with a file path.
//! - `LoaderInClouds`: A structure to thandle loading data in cloud services.
//! - `LoaderFromClouds`: A structure to handle loading data from cloud services..
//! 
//! # Functions
//!
//! - `are_all_clouds_alive`: Checks the availability of multiple cloud storage services using their OAuth tokens.

use std::{collections::HashMap, fs::File, path::Path, str::Bytes};

/// Enum representing different cloud storage services.
pub enum Cloud {
    /// Represents Google Drive.
    GoogleDrive,
    /// Represents Yandex Disk.
    YandexDisk,
    /// Represents a user-defined cloud storage service.
    UserCloud {
        /// The address of the user-defined cloud storage service.
        address: String,
        oauth_path: String,
    },
}

/// Structure to hold an OAuth token.
/// 
/// # Fields
/// 
/// - `token`: A string slice representing the OAuth token.
pub struct OAuthToken<'a> {
    pub token: &'a str,
}

/// Checks the availability of multiple cloud storage services using their OAuth tokens.
///
/// # Arguments
///
/// - `clouds_with_auth`: A slice of tuples, where each tuple contains a `Cloud` enum and an `OAuthToken`.
///
/// # Returns
///
/// - `bool`: Returns `true` if all cloud services are alive, `false` otherwise.
///
/// # Examples
///
/// ```
/// use core::{Cloud, OAuthToken, are_all_clouds_alive};
///
/// let google_token = OAuthToken { token: "google_oauth_token" };
/// let yandex_token = OAuthToken { token: "yandex_oauth_token" };
/// let clouds = vec![
///     (Cloud::GoogleDrive, google_token),
///     (Cloud::YandexDisk, yandex_token),
/// ];
///
/// let all_alive = are_all_clouds_alive(&clouds);
/// assert!(all_alive);
/// ```
pub fn are_all_clouds_alive(clouds_with_auth: &[(Cloud, OAuthToken)]) -> bool {
    // TODO: Implement the actual logic to check cloud service availability.

    bool::default()
}

/// Structure to associate a cloud service with a file path.
/// 
/// # Fields
/// 
/// - `cloud`: The cloud storage service.
/// - `path`: The file path associated with the cloud storage service.
pub struct AddressFile<'p> {
    pub cloud: Cloud,
    pub path: &'p Path,
}

/// Structure to thandle loading data in cloud services.
/// 
/// # Fields
/// 
/// - `use_clouds`: A slice of `Cloud` enums representing the cloud services to be used.
pub struct LoaderInClouds<'a> {
    use_clouds: &'a [Cloud]
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
        Self {
            use_clouds,
        }
    }

    /// Uploads files to the specified cloud storage. At the same time, files can be placed in different clouds.
    /// 
    /// # Arguments
    /// - `loading_files`: All files that will be uploaded.
    /// 
    /// # Returns
    /// - `Vec<AddressFile>`: All addresses of successfully uploaded files.
    pub fn upload_files<'p>(self, loading_files: &[File]) -> Vec<AddressFile<'p>> {
        // TODO: Implement the actual logic to load data in clouds.

        vec![AddressFile {
            cloud: Cloud::GoogleDrive,
            path: Path::new(""),
        }]
    }
}

/// Structure to thandle loading data from cloud services.
/// 
/// # Fields
/// 
/// - `hash_got_file`: Hash where you can get the contents of the `AddressFile`.
pub struct LoaderFromCloud<'a> {
    hash_got_file: HashMap<AddressFile<'a>, Bytes<'a>>
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
    pub fn get_data<'p>(self, address_files: &[AddressFile<'p>]) -> Vec<File> {
        // TODO: Implement the actual logic to load data from clouds.

        let file = File::open("too.txt").unwrap();
        vec![file]
    }
}
