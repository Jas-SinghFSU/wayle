//! Desktop notification service implementing the freedesktop.org Desktop Notifications spec.
//!
//! # Overview
//!
//! Registers as `org.freedesktop.Notifications` on D-Bus to receive notifications from
//! applications. Notifications are stored, displayed as popups, and can be dismissed
//! or have actions invoked.
//!
//! # Reactive Properties
//!
//! Service state is exposed through [`Property`](wayle_common::Property) fields:
//! - `.get()` returns a snapshot of the current value
//! - `.watch()` returns a stream that yields on changes
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `notifications` | `Vec<Arc<Notification>>` | All received notifications |
//! | `popups` | `Vec<Arc<Notification>>` | Currently visible popups |
//! | `popup_duration` | `u32` | Popup display time in ms |
//! | `dnd` | `bool` | Do Not Disturb mode (suppresses popups) |
//! | `remove_expired` | `bool` | Auto-remove expired notifications |
//!
//! # Example
//!
//! ```no_run
//! use wayle_notification::NotificationService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_notification::Error> {
//! let service = NotificationService::new().await?;
//!
//! // Snapshot access
//! let count = service.notifications.get().len();
//!
//! // Reactive stream
//! let mut stream = service.notifications.watch();
//! while let Some(notifications) = stream.next().await {
//!     println!("{} notifications", notifications.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `with_daemon()` | Control notifications from scripts or other processes |
//!
//! ```no_run
//! use wayle_notification::NotificationService;
//!
//! # async fn example() -> Result<(), wayle_notification::Error> {
//! let service = NotificationService::builder()
//!     .with_daemon()
//!     .build()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # D-Bus Interface
//!
//! When `with_daemon()` is enabled, the service registers on the session bus.
//!
//! - **Service:** `com.wayle.Notifications1`
//! - **Path:** `/com/wayle/Notifications`
//! - **Interface:** `com.wayle.Notifications1`
//!
//! See [`dbus.md`](https://github.com/Jas-SinghFSU/wayle/blob/master/crates/wayle-notification/dbus.md) for the full interface specification.

mod builder;
/// Notification data structures and operations.
pub mod core;
pub(crate) mod daemon;
/// Error types.
pub mod error;
pub(crate) mod events;
pub(crate) mod monitoring;
pub(crate) mod persistence;
pub(crate) mod proxy;
/// Service implementation.
pub mod service;
/// freedesktop notification types (Urgency, ClosedReason, Capabilities, etc.).
pub mod types;
pub(crate) mod wayle_daemon;
mod wayle_proxy;

pub use builder::NotificationServiceBuilder;
pub use error::Error;
pub use service::NotificationService;
pub use wayle_proxy::WayleNotificationsProxy;
