use futures::future;

use super::CheckerCloud;
use crate::{
    domain::{Cloud, OAuthToken},
    prelude::CheckAliveResult,
    CheckAliveError,
};

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
    clouds_with_auth: &[Cloud],
) -> Vec<CheckAliveResult<OAuthToken>>
where
    C: CheckerCloud + Copy + Send + 'static,
{
    let tasks: Vec<_> = clouds_with_auth
        .iter()
        .cloned()
        .map(|cloud| tokio::task::spawn(async move { is_cloud_alive(checker, cloud).await }))
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
pub async fn is_cloud_alive<C>(checker: C, cloud: Cloud) -> CheckAliveResult<OAuthToken>
where
    C: CheckerCloud,
{
    checker.check(cloud).await
}
