use super::types::{device::DeviceType, stream::StreamType};

/// PulseAudio service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// PulseAudio connection failed
    #[error("PulseAudio connection failed: {0:#?}")]
    ConnectionFailed(String),

    /// Device not found
    #[error("Device {index:?} ({device_type:?}) not found")]
    DeviceNotFound {
        /// Device index that was not found
        index: u32,
        /// Type of device (input/output)
        device_type: DeviceType,
    },

    /// Stream not found
    #[error("Stream {index:?} ({stream_type:?}) not found")]
    StreamNotFound {
        /// Stream index that was not found
        index: u32,
        /// Type of stream
        stream_type: StreamType,
    },

    /// Command channel disconnected
    #[error("command channel disconnected: {0:#?}")]
    CommandChannelDisconnected(String),

    /// Lock poisoned due to panic in another thread
    #[error("Shared data lock poisoned: {0:#?}")]
    LockPoisoned(String),

    /// Monitoring not initialized - missing required components for live monitoring
    #[error("Monitoring not initialized: {0:#?}")]
    MonitoringNotInitialized(String),
}
