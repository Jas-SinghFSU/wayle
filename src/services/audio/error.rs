use super::{
    types::{device::DeviceType, stream::StreamType},
    volume,
};

/// PulseAudio service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// PulseAudio connection failed
    #[error("PulseAudio connection failed: {0}")]
    ConnectionFailed(String),

    /// PulseAudio operation failed
    #[error("PulseAudio operation failed: {0}")]
    OperationFailed(String),

    /// Volume conversion failed
    #[error("Volume conversion failed")]
    VolumeConversion(#[from] volume::Error),

    /// Volume exceeds safe limits
    #[error(
        "Volume {0} exceeds safe limit of 2.0 (use Volume::with_amplification for higher values)"
    )]
    VolumeExceedsSafeLimit(f64),

    /// Device not found
    #[error("Device {0:?} ({1:?}) not found")]
    DeviceNotFound(u32, DeviceType),

    /// Stream not found
    #[error("Stream {0:?} not found")]
    StreamNotFound(u32, StreamType),

    /// Command channel disconnected
    #[error("command channel disconnected: {0}")]
    CommandChannelDisconnected(String),

    /// Lock poisoned due to panic in another thread
    #[error("Shared data lock poisoned: {0}")]
    LockPoisoned(String),

    /// Service initialization failed
    #[error("Service initialization failed: {0}")]
    InitializationFailed(String),

    /// Backend communication failed
    #[error("backend communication failed: {0}")]
    BackendCommunicationFailed(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),
}
