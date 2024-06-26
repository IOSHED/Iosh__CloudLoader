
#![allow(clippy::all)]

mod api_manager;
mod auth;
mod types;
mod error;
mod prelude;

pub mod config;

pub use types::{AddressFile, Cloud, OAuthToken};
pub use error::AuthError;
pub use prelude::AuthResult;
pub use auth::{CheckerAlive, NetAuthCloud};

