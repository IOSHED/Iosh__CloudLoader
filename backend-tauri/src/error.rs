use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum ErrorController {
    #[error("There is no cloud named {0}.")]
    ThisNameCloudNotFound(String),
    #[error("There is no repository named {0}.")]
    ThisNameRepositoryNotFound(String),
    #[error("An unknown error has occurred: {0}.")]
    Unknow(String),
}

pub type ControllerResult<T, E = ErrorController> = Result<T, E>;
