use super::types::PlayerId;

/// Errors that can occur during media operations
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Player with the given ID was not found
    #[error("player {0} not found")]
    PlayerNotFound(PlayerId),

    /// D-Bus communication error
    #[error("d-bus error: {0}")]
    Dbus(
        #[from]
        #[source]
        zbus::Error,
    ),

    /// Operation not supported by the media player
    #[error("operation not supported: {0}")]
    OperationNotSupported(String),

    /// Cannot initialize the media service
    #[error("cannot initialize media service: {0}")]
    Initialization(String),

    /// Cannot control the player
    #[error("cannot control player: {0}")]
    Control(String),
}
