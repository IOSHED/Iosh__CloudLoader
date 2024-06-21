use async_trait::async_trait;

use crate::{
    api_manager::{google_drive::ApiGoogleDrive, interface::ApiCloud, yandex_disk::ApiYandexDisk},
    domain::{Cloud, OAuthToken},
    prelude::CheckAliveResult,
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
    async fn check(&self, cloud: Cloud) -> CheckAliveResult<OAuthToken>;
}

/// The real implementation of the trait `CheckerCloud`, which sends an authorization request to the cloud given to it.
#[derive(Clone, Copy)]
pub struct NetCheckerCloud;

unsafe impl Send for NetCheckerCloud {}

#[async_trait]
impl CheckerCloud for NetCheckerCloud {
    async fn check(&self, cloud: Cloud) -> CheckAliveResult<OAuthToken> {
        match cloud {
            Cloud::GoogleDrive => ApiGoogleDrive.verify().await,
            Cloud::YandexDisk => ApiYandexDisk.verify().await,
        }
    }
}
