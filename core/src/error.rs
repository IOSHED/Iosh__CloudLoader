use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthError {
    #[error("Tokio tasks connected unsuccessfully.")]
    JoinTokioTaskFailed,

    #[error("Could not access the Auth URL.")]
    FailedGotAuthUrl,

    #[error("Could not access the Tocken URL.")]
    FailedGotTockenUrl,

    #[error("Could not access the Redirect URL.")]
    FailedGotRedirectUrl,

    #[error("Could not access the Tocken Result of User.")]
    FailedGetTockenResult,

    #[error("Server startup error. Try changing the port.")]
    FailedBindServer,

    #[error("Failed to receive token.")]
    FailedReceiveToken,

    #[error("An unknown error has occurred: {0}.")]
    Unknow(String),
}
