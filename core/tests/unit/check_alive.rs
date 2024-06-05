use core::tests::{CheckerCloud, Cloud, OAuthToken, is_cloud_alive, are_all_clouds_alive};

/// Returns `false`` when checking if the token starts with "fake_" otherwise `true`.
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