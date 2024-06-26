#![allow(clippy::all)]

mod api_manager;
mod auth;
mod error;
mod prelude;
mod types;

pub mod config;

pub use auth::{CheckerAlive, NetAuthCloud};
pub use error::AuthError;
pub use prelude::AuthResult;
pub use types::{AddressFile, Cloud, OAuthToken};
