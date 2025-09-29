//! Network device management and monitoring service.
//!
//! This crate provides a service for managing network devices through NetworkManager,
//! including WiFi and wired connections. It monitors connection state, device
//! availability, and network events, exposing device information and state changes
//! through a reactive stream-based API.

/// Core network domain models
pub mod core;
mod discovery;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Network type definitions
pub mod types;
/// WiFi device functionality
pub mod wifi;
/// Wired device functionality
pub mod wired;

pub use error::Error;
pub use service::NetworkService;
