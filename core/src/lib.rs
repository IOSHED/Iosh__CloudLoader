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
//! - `LoaderFromClouds`: A structure to handle loading data from cloud services.
//!
//! # Functions
//!
//! - `are_all_clouds_alive`: Checks the availability of multiple cloud storage services using their OAuth tokens.
//! - `is_cloud_alive`
//!
//! # Modules
//!
//! - `error`
//!

#![allow(clippy::all)]

mod check_alive;
mod domain;
mod error;
mod loaders;
mod prelude;

pub mod tests {
   pub use super::check_alive::{CheckerCloud, is_cloud_alive, are_all_clouds_alive};
   pub use super::domain::{AddressFile, Cloud, OAuthToken};
} 

pub use domain::{Cloud, OAuthToken};

/// Checks the availability of multiple cloud storage services using their OAuth tokens.
///
/// # Arguments
///
/// - `clouds_with_auth`: A slice of tuples, where each tuple contains a `Cloud` enum and an `OAuthToken`.
///
/// # Returns
///
/// - `Vec<bool>`: Returns an array of bool values. Each index of the returned array corresponds to the index of the `clouds_with_auth argument`.
///
/// # Examples
///
/// ```
/// use core::{Cloud, OAuthToken, are_all_clouds_alive};
///
/// let google_token = OAuthToken { token: "fake_google_oauth_token" };
/// let yandex_token = OAuthToken { token: "fake_yandex_oauth_token" };
/// let clouds = vec![
///     (Cloud::GoogleDrive, google_token),
///     (Cloud::YandexDisk, yandex_token),
/// ];
///
/// let all_alive = are_all_clouds_alive(&clouds);
/// assert_eq!(all_alive, [false, false]);
/// ```
pub async fn are_all_clouds_alive(clouds_with_auth: &[(Cloud, OAuthToken)]) -> Vec<bool> {
    use check_alive::NetCheckerCloud;
    check_alive::are_all_clouds_alive(NetCheckerCloud, clouds_with_auth).await
}

/// Checks the availability of cloud storage service.
///
/// # Arguments
///
/// - `cloud`: The cloud that is being checked for performance.
/// - `tocken`: OAuth token for authorization on this cloud.
///
/// # Returns
///
/// - `bool`: Returns `true` if cloud service is alive, `false` otherwise.
///
/// # Examples
/// ```
/// use core::{Cloud, OAuthToken, is_cloud_alive};
///
/// let google_token = OAuthToken { token: "fake_google_oauth_token" };
///
/// let is_alive = is_cloud_alive(&Cloud::GoogleDrive, &google_token);
/// assert_eq!(is_alive, false);
/// ```
pub async fn is_cloud_alive(cloud: Cloud, tocken: OAuthToken) -> bool {
    use check_alive::NetCheckerCloud;
    check_alive::is_cloud_alive(NetCheckerCloud, cloud, tocken).await
}
