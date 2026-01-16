//! Battery monitoring service for power devices via UPower.
//!
//! Tracks battery levels, charging state, and power events, exposing device
//! information and state changes through a reactive stream-based API.

mod builder;
/// Core battery device functionality.
pub mod core;
mod error;
mod proxy;
mod service;
/// Type definitions for battery service domain models and enums.
pub mod types;

pub use builder::BatteryServiceBuilder;
pub use error::Error;
pub use service::BatteryService;
