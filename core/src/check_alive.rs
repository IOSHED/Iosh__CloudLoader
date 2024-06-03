use crate::domain::{Cloud, OAuthToken};

use futures::stream::FuturesUnordered;

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
pub async fn are_all_clouds_alive(clouds_with_auth: &[(Cloud, OAuthToken)]) -> bool {
    let mut tasks = vec![];

    for (cloud, token) in clouds_with_auth.into_iter().cloned() {
        let task = tokio::task::spawn(async move { is_cloud_alive(&cloud, &token).await });
        tasks.push(task);
    }

    let result = futures::future::join_all(tasks).await;

    result.into_iter().all(|value| value.unwrap())
}

pub async fn is_cloud_alive(cloud: &Cloud, tocken: &OAuthToken) -> bool {
    match cloud {
        Cloud::GoogleDrive => is_google_frive_alive(tocken).await,
        Cloud::YandexDisk => is_yandex_disk_alive(tocken).await,
        Cloud::UserCloud {
            address,
            oauth_path,
        } => is_user_cloud_alive(address, oauth_path, tocken).await,
    }
}

async fn is_google_frive_alive(tocken: &OAuthToken) -> bool {
    bool::default()
}

async fn is_yandex_disk_alive(tocken: &OAuthToken) -> bool {
    bool::default()
}

async fn is_user_cloud_alive(address: &str, oauth_path: &str, tocken: &OAuthToken) -> bool {
    bool::default()
}
