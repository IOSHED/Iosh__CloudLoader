use async_trait::async_trait;
use futures::future;

use crate::{
    domain::{Cloud, OAuthToken},
    prelude::CheckAliveResult,
    CheckAliveError,
};

/// Describes what kind of verification needs to be carried out real or fake.
#[async_trait]
pub trait CheckerCloud {
    /// Verifies the authenticity of the OAuth token for a specific cloud.
    ///
    /// # Arguments
    ///
    /// - `cloud`: The cloud that is being checked for performance.
    /// - `tocken`: OAuth token for authorization on this cloud.
    ///
    /// # Return
    ///
    /// - `CheckAliveResult<bool>`: Returns `true` if cloud service is alive, `false` (or Error telling you why the token verification is not possible) otherwise.
    async fn check(
        &self,
        cloud: Cloud,
        token: OAuthToken,
    ) -> CheckAliveResult<bool>;
}

/// The real implementation of the trait `CheckerCloud`, which sends an authorization request to the cloud given to it.
#[derive(Clone, Copy)]
pub struct NetCheckerCloud;

unsafe impl Send for NetCheckerCloud {}

impl NetCheckerCloud {
    async fn verify_google_drive(token: &OAuthToken) -> CheckAliveResult<bool> {
        // Implementation to verify Google Drive token
        Ok(true) // Placeholder for actual implementation
    }

    async fn verify_yandex_disk(token: &OAuthToken) -> CheckAliveResult<bool> {
        // Implementation to verify Yandex Disk token
        Ok(true) // Placeholder for actual implementation
    }
}

#[async_trait]
impl CheckerCloud for NetCheckerCloud {
    async fn check(
        &self,
        cloud: Cloud,
        token: OAuthToken,
    ) -> CheckAliveResult<bool> {

        match cloud {
            Cloud::GoogleDrive => NetCheckerCloud::verify_google_drive(&token).await,
            Cloud::YandexDisk => NetCheckerCloud::verify_yandex_disk(&token).await,
        }
    }
}

/// Checks the availability of multiple cloud storage services using their OAuth tokens.
///
/// # Arguments
///
/// - `checker`: The structure implementing the trate `CheckerCloud`, which checks for the correctness of the OAuth token.
/// - `clouds_with_auth`: A slice of tuples, where each tuple contains a `Cloud` enum and an `OAuthToken`.
///
/// # Returns
///
/// - `Vec<CheckAliveResult<bool>>`: Returns an array of bool values (or Error telling you why the token verification is not possible). Each index of the returned array corresponds to the index of the `clouds_with_auth argument`.
///
/// # Examples
///
/// ```no_run
/// use core::check_alive::{NetCheckerCloud, Cloud, OAuthToken, are_all_clouds_alive};
///
/// let google_token = OAuthToken { token: "fake_google_oauth_token" };
/// let yandex_token = OAuthToken { token: "fake_yandex_oauth_token" };
/// let clouds = vec![
///     (Cloud::GoogleDrive, google_token),
///     (Cloud::YandexDisk, yandex_token),
/// ];
///
/// let all_alive = are_all_clouds_alive(NetCheckerCloud, &clouds);
/// assert_eq!(all_alive, [Ok(false), Ok(false)]);
/// ```
pub async fn are_all_clouds_alive<C>(
    checker: C,
    clouds_with_auth: &[(Cloud, OAuthToken)],
) -> Vec<CheckAliveResult<bool>>
where
    C: CheckerCloud + Copy + Send + 'static,
{
    let tasks: Vec<_> = clouds_with_auth
        .iter()
        .cloned()
        .map(|(cloud, token)| {
            tokio::task::spawn(async move { is_cloud_alive(checker, cloud, token).await })
        })
        .collect();

    let results = future::join_all(tasks).await;
    results
        .into_iter()
        .map(|res| res.unwrap_or(Err(CheckAliveError::JoinTokioTaskFailed)))
        .collect()
}

/// Checks the availability of cloud storage service.
///
/// # Arguments
///
/// - `checker`: The structure implementing the trate `CheckerCloud`, which checks for the correctness of the OAuth token.
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
/// use core::check_alive::{NetCheckerCloud, Cloud, OAuthToken, is_cloud_alive};
///
/// let google_token = OAuthToken { token: "fake_google_oauth_token" };
///
/// let is_alive = is_cloud_alive(NetCheckerCloud, Cloud::GoogleDrive, google_token);
/// assert_eq!(is_alive, Ok(false));
/// ```
pub async fn is_cloud_alive<C>(
    checker: C,
    cloud: Cloud,
    token: OAuthToken,
) -> CheckAliveResult<bool>
where
    C: CheckerCloud,
{
    checker.check(cloud, token).await
}
