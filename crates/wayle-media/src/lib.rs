//! Media service for monitoring and controlling media players via MPRIS.
//!
//! Provides a unified interface for interacting with media players that
//! implement the MPRIS D-Bus specification, including playback control,
//! metadata retrieval, and state monitoring.

mod builder;
/// Core media domain models
pub mod core;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Type definitions for media service configuration, states, and identifiers
pub mod types;

pub use builder::MediaServiceBuilder;
pub use error::Error;
pub use service::MediaService;
