/// Enum representing different cloud storage services.
#[derive(Clone)]
pub enum Cloud {
    /// Represents Google Drive.
    GoogleDrive,
    /// Represents Yandex Disk.
    YandexDisk,
    /// Represents a user-defined cloud storage service.
    UserCloud {
        /// The address of the user-defined cloud storage service.
        address: String,
        /// The path where the API request for token authentication will be sent.
        oauth_path: String,
    },
}
