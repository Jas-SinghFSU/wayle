//! Desktop notification management service.
//!
//! Manages desktop notifications through the freedesktop.org Desktop Notifications
//! Specification. Handles notification display, persistence, and reactive streams
//! for notification events and state changes.

mod builder;
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
pub(crate) mod wayle_daemon;
mod wayle_proxy;

pub use builder::NotificationServiceBuilder;
pub use error::Error;
pub use service::NotificationService;
pub use wayle_proxy::WayleNotificationsProxy;
