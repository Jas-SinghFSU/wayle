//! Bluetooth device management and discovery service.
//!
//! This crate provides a service for managing Bluetooth devices through BlueZ,
//! including device discovery, pairing, and connection monitoring. It exposes
//! device information and state changes through a reactive stream-based API.

mod agent;
/// Core Bluetooth domain models
pub mod core;
mod discovery;
mod error;
mod monitoring;
mod proxy;
mod service;
/// Bluetooth type definitions
pub mod types;

pub use error::Error;
pub use service::BluetoothService;
