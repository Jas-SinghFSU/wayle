/// Core types and functionality for system tray items
pub mod core;
mod discovery;
/// Error types for the system tray service
pub mod error;
/// Event types for system tray state changes
pub mod events;
/// Service-level monitoring for host mode
pub mod monitoring;
/// D-Bus proxy trait definitions
pub mod proxy;
/// Main system tray service implementation
pub mod service;
/// Type definitions for StatusNotifier and DBusMenu protocols
pub mod types;
/// StatusNotifierWatcher implementation
pub mod watcher;

pub use service::{SystemTrayService, SystemTrayServiceBuilder};
