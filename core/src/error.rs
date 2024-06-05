#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Failed to check cloud service availability for {0}")]
    ServiceUnavailable(String),
    #[error("Unknown cloud service")]
    UnknownService,
}
