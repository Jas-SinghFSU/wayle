//! Battery monitoring service for power devices.
//!
//! This crate provides a service for monitoring battery and power devices
//! through UPower. It tracks battery levels, charging state, and power events,
//! exposing device information and state changes through a reactive stream-based API.

/// Battery service for monitoring power devices via UPower
pub mod core;
mod error;
mod proxy;
mod service;
/// Type definitions for battery service domain models and enums
pub mod types;

pub use error::Error;
pub use service::{BatteryService, BatteryServiceBuilder};
