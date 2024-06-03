use std::path::Path;

use super::Cloud;

/// Structure to associate a cloud service with a file path.
///
/// # Fields
///
/// - `cloud`: The cloud storage service.
/// - `path`: The file path associated with the cloud storage service.
pub struct AddressFile<'p> {
    pub cloud: Cloud,
    pub path: &'p Path,
}
