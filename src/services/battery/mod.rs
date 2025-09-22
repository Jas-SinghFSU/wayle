/// Battery service for monitoring power devices via UPower
pub mod core;
mod error;
mod proxy;
mod service;
/// Type definitions for battery service domain models and enums
pub mod types;

pub use error::Error;
pub use service::{BatteryService, BatteryServiceBuilder};
