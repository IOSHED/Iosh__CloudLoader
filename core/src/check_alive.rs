use std::pin::Pin;

use crate::domain::{Cloud, OAuthToken};

use futures::Future;

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
    /// - `Pin<Box<dyn Future<Output = bool> + Send>>`: Returns `true` if cloud service is alive, `false` otherwise.
    /// 
    fn check(&self, cloud: Cloud, token: OAuthToken) -> Pin<Box<dyn Future<Output = bool> + Send>>;
}

/// The real implementation of the trait `CheckerCloud`, which sends an authorization request to the cloud given to it.
#[derive(Clone, Copy)]
pub struct NetCheckerCloud;

unsafe impl Send for NetCheckerCloud {}

impl CheckerCloud for NetCheckerCloud {
    fn check(&self, _cloud: Cloud, _token: OAuthToken) -> Pin<Box<dyn Future<Output = bool> + Send>> {
        // TODO: Implement real logic
        Box::pin(async { true })
    }
}

pub async fn are_all_clouds_alive<C>(checker: C, clouds_with_auth: &[(Cloud, OAuthToken)]) -> Vec<bool> 
where
    C: CheckerCloud + Copy + Send + 'static,
{
    let tasks: Vec<_> = clouds_with_auth.iter().cloned().map(|(cloud, token)| {
        tokio::task::spawn(async move {
            is_cloud_alive(checker, cloud, token).await
        })
    }).collect();

    let results = futures::future::join_all(tasks).await;
    results.into_iter().map(|res| res.unwrap()).collect()
}

pub async fn is_cloud_alive<C>(checker: C, cloud: Cloud, token: OAuthToken) -> bool 
where
    C: CheckerCloud,
{
    checker.check(cloud, token).await
}

#[cfg(test)]
mod tests {
    use super::{are_all_clouds_alive, is_cloud_alive, CheckerCloud, Cloud, OAuthToken};

    /// Returns `false`` when checking if the token starts with "fake_" otherwise `true`.
    #[cfg(test)]
    #[derive(Clone, Copy)]
    struct FakeCheckerCloud;

    unsafe impl Send for FakeCheckerCloud {}

    impl CheckerCloud for FakeCheckerCloud {
        fn check(&self, _cloud: Cloud, token: OAuthToken) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send>> {
            Box::pin(async move { !(token.token.get(0..=4) == Some("fake_")) })
        }
    }

    #[tokio::test]
    async fn test_check_true_result_is_cloud_alive() {
        let checker = FakeCheckerCloud;
        let cloud = Cloud::GoogleDrive;
        let tocken = OAuthToken { token: "true_tocken".to_string() };

        let result = is_cloud_alive(checker, cloud, tocken).await;

        assert_eq!(result, true, "Checking for the returned true result function `is_cloud_alive`.")
    }

    #[tokio::test]
    async fn test_check_false_result_is_cloud_alive() {

        let checker = FakeCheckerCloud;
        let cloud = Cloud::GoogleDrive;
        let tocken = OAuthToken { token: "fake_tocken".to_string() };

        let result = is_cloud_alive(checker, cloud, tocken).await;

        assert_eq!(result, false, "Checking for the returned false result function `is_cloud_alive`.");
    }

    #[tokio::test]
    async fn test_check_all_true_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (Cloud::GoogleDrive, OAuthToken { token: "true_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "true_tocken".to_string() }),
            (Cloud::GoogleDrive, OAuthToken { token: "true_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "true_tocken".to_string() }),
        ];
        
        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(clouds_with_auth.len(), result.len(), "Checking that the number of returned items is equal to the number of passed items.");
        assert_eq!(result, vec![true].repeat(result.len()), "Checking that all Oauth tokens are true.");
    }

    #[tokio::test]
    async fn test_check_all_false_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (Cloud::GoogleDrive, OAuthToken { token: "fake_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "fake_tocken".to_string() }),
            (Cloud::GoogleDrive, OAuthToken { token: "fake_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "fake_tocken".to_string() }),
        ];
        
        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(clouds_with_auth.len(), result.len(), "Checking that the number of returned items is equal to the number of passed items.");
        assert_eq!(result, vec![false].repeat(result.len()), "Checking that all Oauth tokens are fake.");
    }

    #[tokio::test]
    async fn test_check_with_different_tokens_are_all_clouds_alive() {
        let checker = FakeCheckerCloud;
        let clouds_with_auth = vec![
            (Cloud::GoogleDrive, OAuthToken { token: "fake_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "true_tocken".to_string() }),
            (Cloud::GoogleDrive, OAuthToken { token: "true_tocken".to_string() }),
            (Cloud::YandexDisk, OAuthToken { token: "fake_tocken".to_string() }),
        ];
        
        let result = are_all_clouds_alive(checker, &clouds_with_auth).await;

        assert_eq!(clouds_with_auth.len(), result.len(), "Checking that the number of returned items is equal to the number of passed items.");
        assert_eq!(result, vec![false, true, true, false], "Checking that the returned values correspond to the correctness of the transferred tokens.");
    }
}