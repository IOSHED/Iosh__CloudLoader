use crate::error::AuthError;

pub type AuthResult<T> = Result<T, AuthError>;
