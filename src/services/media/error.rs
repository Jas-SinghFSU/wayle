use super::types::PlayerId;

/// Errors that can occur during media operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Player with the given ID was not found
    #[error("Player {0:?} not found")]
    PlayerNotFound(PlayerId),

    /// D-Bus communication error
    #[error("D-Bus operation failed: {0:#?}")]
    DbusError(#[from] zbus::Error),

    /// Operation not supported (simplified version)
    #[error("Operation not supported: {0:#?}")]
    OperationNotSupported(String),

    /// Failed to initialize the media service
    #[error("Failed to initialize media service: {0:#?}")]
    InitializationFailed(String),

    /// Failed to control the player
    #[error("Failed to control player: {0:#?}")]
    ControlFailed(String),
}
