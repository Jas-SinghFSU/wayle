/// Core types and functionality for system tray items
pub mod core;
mod discovery;
/// Error types for the system tray service
pub mod error;
mod events;
mod monitoring;
mod proxy;
/// Main system tray service implementation
pub mod service;
/// Type definitions for StatusNotifier and DBusMenu protocols
pub mod types;
mod watcher;

pub use service::{SystemTrayService, SystemTrayServiceBuilder};
