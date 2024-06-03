//! This module provides an enumeration for different cloud storage services, structures for OAuth tokens and file addresses, and functions to check cloud service availability and load data from clouds.
//!
//! # Enums
//!
//! - `Cloud`: Represents different cloud storage services.
//!
//! # Structs
//!
//! - `OAuthToken`: A structure to hold an OAuth token as a string slice.
//! - `AddressFile`: A structure to associate a cloud service with a file path.
//! - `LoaderInClouds`: A structure to thandle loading data in cloud services.
//! - `LoaderFromClouds`: A structure to handle loading data from cloud services.
//!
//! # Functions
//!
//! - `are_all_clouds_alive`: Checks the availability of multiple cloud storage services using their OAuth tokens.
//! - `is_cloud_alive`
//!
//! # Modules
//!
//! - `error`
//!

#![allow(clippy::all)]

mod check_alive;
mod domain;
mod error;
mod loaders;

// TODO: import all for public using
