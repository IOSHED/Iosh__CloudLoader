use std::pin::Pin;

use futures::{future, Future};

use crate::{
    domain::{Cloud, OAuthToken},
    prelude::CheckAliveResult,
    CheckAliveError,
};

/// Describes what kind of verification needs to be carried out real or fake.
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
    /// - `Pin<Box<dyn Future<Output = CheckAliveResult<bool>> + Send>>`: Returns `true` if cloud service is alive, `false` (or Error telling you why the token verification is not possible) otherwise.
    fn check(
        &self,
        cloud: Cloud,
        token: OAuthToken,
    ) -> Pin<Box<dyn Future<Output = CheckAliveResult<bool>> + Send>>;
}

/// The real implementation of the trait `CheckerCloud`, which sends an authorization request to the cloud given to it.
#[derive(Clone, Copy)]
pub struct NetCheckerCloud;

unsafe impl Send for NetCheckerCloud {}

impl CheckerCloud for NetCheckerCloud {
    fn check(
        &self,
        _cloud: Cloud,
        _token: OAuthToken,
    ) -> Pin<Box<dyn Future<Output = CheckAliveResult<bool>> + Send>> {
        // TODO: Implement real logic
        Box::pin(async { Ok(true) })
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

#[cfg(test)]
mod tests {
    use std::pin::Pin;

    use futures::Future;

    use super::{are_all_clouds_alive, is_cloud_alive, CheckerCloud, Cloud, OAuthToken};
    use crate::prelude::CheckAliveResult;

    /// Returns `false`` when checking if the token starts with "fake_" otherwise `true`.
    #[derive(Clone, Copy)]
    struct FakeCheckerCloud;

    unsafe impl Send for FakeCheckerCloud {}

    impl CheckerCloud for FakeCheckerCloud {
        fn check(
            &self,
            _cloud: Cloud,
            token: OAuthToken,
        ) -> Pin<Box<dyn Future<Output = CheckAliveResult<bool>> + Send>> {
            Box::pin(async move { Ok(!(token.token.get(0..=4) == Some("fake_"))) })
        }
    }

    #[tokio::test]
    async fn test_check_true_result_is_cloud_alive() {
        let checker = FakeCheckerCloud;
        let cloud = Cloud::GoogleDrive;
        let tocken = OAuthToken {
            token: "true_tocken".to_string(),
        };

        let result = is_cloud_alive(checker, cloud, tocken).await;

        assert_eq!(
            result,
            Ok(true),
            "Checking for the returned true result function `is_cloud_alive`."
        )
    }

    #[tokio::test]
    async fn test_check_false_result_is_cloud_alive() {
        let checker = FakeCheckerCloud;
        let cloud = Cloud::GoogleDrive;
        let tocken = OAuthToken {
            token: "fake_tocken".to_string(),
        };

        let result = is_cloud_alive(checker, cloud, tocken).await;

        assert_eq!(
            result,
            Ok(false),
            "Checking for the returned false result function `is_cloud_alive`."
        );
    }

    #[tokio::test]
    async fn test_check_all_true_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
        ];

        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(
            clouds_with_auth.len(),
            result.len(),
            "Checking that the number of returned items is equal to the number of passed items."
        );
        assert_eq!(
            result,
            vec![Ok(true)].repeat(result.len()),
            "Checking that all Oauth tokens are true."
        );
    }

    #[tokio::test]
    async fn test_check_all_false_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
        ];

        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(
            clouds_with_auth.len(),
            result.len(),
            "Checking that the number of returned items is equal to the number of passed items."
        );
        assert_eq!(
            result,
            vec![Ok(false)].repeat(result.len()),
            "Checking that all Oauth tokens are fake."
        );
    }

    #[tokio::test]
    async fn test_check_with_different_tokens_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
            (
                Cloud::GoogleDrive,
                OAuthToken {
                    token: "true_tocken".to_string(),
                },
            ),
            (
                Cloud::YandexDisk,
                OAuthToken {
                    token: "fake_tocken".to_string(),
                },
            ),
        ];

        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(
            clouds_with_auth.len(),
            result.len(),
            "Checking that the number of returned items is equal to the number of passed items."
        );
        assert_eq!(result, vec![Ok(false), Ok(true), Ok(true), Ok(false)], "Checking that the returned values correspond to the correctness of the transferred tokens.");
    }
}
