//! Audio service for managing system sound devices and streams.
//!
//! Provides integration with PulseAudio to monitor and control audio devices,
//! including input/output device management, volume control, and stream monitoring.

#![cfg_attr(test, allow(clippy::panic))]

mod backend;
mod builder;
/// Core domain models
pub mod core;
/// D-Bus interface for external control
pub mod dbus;
mod error;
mod events;
mod monitoring;
mod service;
mod tokio_mainloop;
/// Types for the audio service
pub mod types;
/// Volume control domain
pub mod volume;

pub use builder::AudioServiceBuilder;
pub use error::Error;
pub use service::AudioService;
