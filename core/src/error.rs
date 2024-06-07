use thiserror::Error;

#[derive(Error, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckAliveError {
    #[error("Tokio tasks connected unsuccessfully.")]
    JoinTokioTaskFailed,

    #[error("An unknown error has occurred: {0}.")]
    Unknow(&'static str),
}
