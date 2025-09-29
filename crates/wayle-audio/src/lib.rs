//! Audio service for managing system sound devices and streams.
//!
//! Provides integration with PulseAudio to monitor and control audio devices,
//! including input/output device management, volume control, and stream monitoring.

mod backend;
/// Core domain models
pub mod core;
mod error;
mod events;
mod monitoring;
mod service;
mod tokio_mainloop;
/// Types for the audio service
pub mod types;
/// Volume control domain
pub mod volume;

pub use error::Error;
pub use service::AudioService;
