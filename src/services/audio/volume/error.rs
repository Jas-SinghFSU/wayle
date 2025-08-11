/// Volume-related errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum VolumeError {
    /// Invalid volume level
    #[error("Invalid volume {volume} for channel {channel} (must be 0.0-10.0)")]
    InvalidVolume {
        /// Channel index
        channel: usize,
        /// Invalid volume value
        volume: f64,
    },
    /// Invalid channel index
    #[error("Invalid channel index {channel}")]
    InvalidChannel {
        /// Channel index
        channel: usize,
    },
}
