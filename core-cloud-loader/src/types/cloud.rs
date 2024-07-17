use serde::{Deserialize, Serialize};

/// Enum representing different cloud storage services.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cloud {
    /// Represents Google Drive.
    GoogleDrive,
    // /// Represents Yandex Disk.
    // YandexDisk,
}
