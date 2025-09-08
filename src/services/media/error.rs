use std::time::Duration;

use super::types::PlayerId;

/// Errors that can occur during media operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Player with the given ID was not found
    #[error("Player {0:?} not found")]
    PlayerNotFound(PlayerId),

    /// D-Bus communication error
    #[error("D-Bus operation failed: {0}")]
    DbusError(#[from] zbus::Error),

    /// Player doesn't support the requested operation
    #[error("Player {player:?} doesn't support {operation}")]
    UnsupportedOperation {
        /// Player that doesn't support the operation
        player: PlayerId,
        /// Name of the unsupported operation
        operation: String,
    },

    /// Operation not supported (simplified version)
    #[error("Operation not supported: {0}")]
    OperationNotSupported(String),

    /// Seek position is invalid for the current track
    #[error("Invalid seek position: {position:?} (track length: {length:?})")]
    InvalidSeekPosition {
        /// Requested position
        position: Duration,
        /// Track length (if known)
        length: Option<Duration>,
    },

    /// Player is not responding to requests
    #[error("Player {0:?} not responding")]
    PlayerUnresponsive(PlayerId),

    /// Failed to initialize the media service
    #[error("Failed to initialize media service: {0}")]
    InitializationFailed(String),

    /// Failed to control the player
    #[error("Failed to control player: {0}")]
    ControlFailed(String),

    /// An invalid input was provided
    #[error("An invalid input was provided: {0:?}")]
    InvalidInput(String),
}
