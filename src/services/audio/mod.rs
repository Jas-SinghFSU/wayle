/// PulseAudio backend implementation
pub mod backend;
/// Core domain models
pub mod core;
/// Discovery functionality
pub mod discovery;
/// Error types
pub mod error;
/// Event types and handling
pub mod events;
/// Audio service implementation
mod service;
/// Tokio mainloop for PulseAudio
pub mod tokio_mainloop;
/// Types for the audio service
pub mod types;
/// Volume control domain
pub mod volume;

pub use core::{AudioStream, InputDevice, OutputDevice};

pub use error::AudioError;
pub use events::AudioEvent;
pub use service::AudioService;
pub use types::{
    AudioFormat, ChannelMap, DeviceInfo, DeviceKey, DevicePort, DeviceState, DeviceType, MediaInfo,
    SampleFormat, SampleSpec, StreamInfo, StreamKey, StreamState, StreamType,
};
pub use volume::{Volume, VolumeError};
