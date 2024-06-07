use crate::error::CheckAliveError;

pub type CheckAliveResult<T> = Result<T, CheckAliveError>;
