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
//!
//! # Functions
//!
//! - `are_all_clouds_alive`: Checks the availability of multiple cloud storage services using their OAuth tokens.
//! - `is_cloud_alive`: Checks the availability of cloud storage service.
//!
//! # Modules
//!
//! - `tests`: the module used to get the functions and structures that will be tested should **not be imported into your projects**.

#![allow(clippy::all)]

mod api_manager;
mod check_alive;
mod domain;
mod error;
mod loaders;
mod prelude;

pub use domain::{AddressFile, Cloud, OAuthToken};
pub use error::CheckAliveError;
pub use prelude::CheckAliveResult;

pub mod tests {
    pub use super::check_alive::{are_all_clouds_alive, is_cloud_alive, CheckerCloud};
    pub use super::domain::{AddressFile, Cloud, OAuthToken};
    pub use super::prelude::CheckAliveResult;
}

/// Checks the availability of multiple cloud storage services using their OAuth tokens.
///
/// # Arguments
///
/// - `clouds_with_auth`: A slice of tuples, where each tuple contains a `Cloud` enum and an `OAuthToken`.
///
/// # Returns
///
/// - `Vec<CheckAliveResult<bool>>`: Returns an array of bool values (or Error telling you why the token verification is not possible). Each index of the returned array corresponds to the index of the `clouds_with_auth argument`.
///
/// # Examples
///
/// ```no_run
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
/// assert_eq!(all_alive, [Ok(false), Ok(false)]);
/// ```
pub async fn are_all_clouds_alive(clouds_with_auth: &[Cloud]) -> Vec<CheckAliveResult<OAuthToken>> {
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
/// - `CheckAliveResult<bool>`: Returns `true` if cloud service is alive, `false` (or Error telling you why the token verification is not possible) otherwise.
///
/// # Examples
///
/// ```no_run
/// use core::{Cloud, OAuthToken, is_cloud_alive};
///
/// let google_token = OAuthToken { token: "fake_google_oauth_token" };
///
/// let is_alive = is_cloud_alive(Cloud::GoogleDrive, google_token);
/// assert_eq!(is_alive, Ok(false));
/// ```
pub async fn is_cloud_alive(cloud: Cloud) -> CheckAliveResult<OAuthToken> {
    use check_alive::NetCheckerCloud;
    check_alive::is_cloud_alive(NetCheckerCloud, cloud).await
}
