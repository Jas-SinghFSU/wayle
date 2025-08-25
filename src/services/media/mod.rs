//! MPRIS-based media service implementation
//!
//! This module provides reactive media player control through the D-Bus MPRIS protocol.
//! It automatically discovers players and provides streams for UI updates.

/// Error types for the MPRIS service
pub mod error;
/// Main MPRIS service implementation
pub mod service;
/// Type definitions for MPRIS functionality
pub mod types;

/// Core MPRIS data models and monitoring
pub mod core;

mod monitoring;
mod proxy;

pub use core::{Player, TrackMetadata, UNKNOWN_METADATA};

pub use error::MediaError;
pub use service::{Config, MediaService};
pub use types::*;
