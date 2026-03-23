//! Backlight control for internal displays.
//!
//! All state is exposed via [`Property`](wayle_core::Property) fields that
//! update automatically when brightness changes.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_brightness::{BrightnessService, Percentage};
//!
//! # async fn example() -> Result<(), wayle_brightness::Error> {
//! let Some(brightness) = BrightnessService::new().await? else {
//!     return Ok(());
//! };
//!
//! if let Some(device) = brightness.primary.get() {
//!     println!("{}: {}", device.name, device.percentage());
//!     device.set_percentage(Percentage::new(50.0)).await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_brightness::BrightnessService;
//! use futures::StreamExt;
//!
//! # async fn example() -> Result<(), wayle_brightness::Error> {
//! # let Some(brightness) = BrightnessService::new().await? else { return Ok(()) };
//! let mut stream = brightness.primary.watch();
//!
//! while let Some(maybe_device) = stream.next().await {
//!     if let Some(device) = maybe_device {
//!         println!("brightness: {}", device.percentage());
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Non-systemd Systems
//!
//! Direct sysfs writes require `video` group membership.

mod backend;
/// [`BrightnessServiceBuilder`] for custom configuration.
pub mod builder;
/// [`BacklightDevice`] and monitoring.
pub mod core;
/// [`Error`] variants for backlight operations.
pub mod error;
mod monitoring;
/// [`BrightnessService`] entry point.
pub mod service;
/// [`BacklightType`](types::BacklightType), [`Percentage`], [`DeviceName`].
pub mod types;

pub use core::BacklightDevice;

pub use builder::BrightnessServiceBuilder;
pub use error::Error;
pub use service::BrightnessService;
pub use types::{DeviceName, Percentage};
