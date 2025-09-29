//! Desktop notification management service.
//!
//! This crate provides a notification service that manages desktop notifications
//! through the freedesktop.org Desktop Notifications Specification. It handles
//! notification display, persistence, and provides reactive streams for
//! notification events and state changes.

/// Core notification functionality.
pub mod core;
pub(crate) mod daemon;
/// Error types for the notification service.
pub mod error;
pub(crate) mod events;
pub(crate) mod monitoring;
pub(crate) mod persistence;
pub(crate) mod proxy;
/// Notification service implementation.
pub mod service;
/// Type definitions for notifications.
pub mod types;
