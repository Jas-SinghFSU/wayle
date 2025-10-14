use std::io;

use tokio::sync::broadcast;

use crate::{Address, HyprlandEvent, WorkspaceId};

/// Hyprland service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Failed to initialize Hyprland service
    #[error("Failed to initialize Hyprland service: {0:#?}")]
    ServiceInitializationFailed(String),

    /// IPC socket connection error
    #[error("Failed to connect to Hyprland IPC socket: {socket_type} - {reason}")]
    IpcConnectionFailed {
        /// Type of socket (command or event)
        socket_type: &'static str,
        /// Reason for connection failure
        reason: String,
    },

    /// Socket I/O operation failed
    #[error("IPC socket I/O error: {0:#?}")]
    SocketIoError(#[from] io::Error),

    /// Failed to parse JSON response from Hyprland
    #[error("Failed to parse Hyprland response: {0:#?}")]
    JsonParseError(#[from] serde_json::Error),

    /// Failed to parse event from Hyprland event socket
    #[error("Failed to parse event: {event_data} - {reason}")]
    EventParseError {
        /// Raw event data that failed to parse
        event_data: String,
        /// Reason for parse failure
        reason: String,
    },

    /// Hyprland command execution failed
    #[error("Command execution failed: {command} - {reason}")]
    CommandFailed {
        /// Command that was executed
        command: String,
        /// Reason for failure
        reason: String,
    },

    /// Workspace with specified ID not found
    #[error("Workspace {0} not found")]
    WorkspaceNotFound(WorkspaceId),

    /// Monitor with specified name not found
    #[error("Monitor {0} not found")]
    MonitorNotFound(String),

    /// Client/Window with specified address not found
    #[error("Client {0} not found")]
    ClientNotFound(Address),

    /// Layer with specified address not found
    #[error("Layer {0} not found")]
    LayerNotFound(Address),

    /// Invalid Hyprland instance signature
    #[error("Invalid Hyprland instance signature: {0:#?}")]
    InvalidInstanceSignature(String),

    /// Hyprland is not running or not accessible
    #[error("Hyprland is not running or HYPRLAND_INSTANCE_SIGNATURE is not set")]
    HyprlandNotRunning,

    /// Operation not supported by current Hyprland version
    #[error("Operation not supported: {0:#?}")]
    OperationNotSupported(String),

    /// Generic operation failure with context
    #[error("Hyprland operation failed: {operation} - {reason}")]
    OperationFailed {
        /// Operation that failed
        operation: &'static str,
        /// Reason for failure
        reason: String,
    },

    /// Failed to transmit hyprland event
    #[error("Failed to transmit hyprland event: {0}")]
    HyprlandEventTransmitError(#[from] broadcast::error::SendError<HyprlandEvent>),

    /// Failed to transmit internal service notification
    #[error("Failed to transmit internal service notification: {0}")]
    InternalEventTransmitError(String),

    /// Invalid value for enum conversion
    #[error("Invalid value for {type_name}: {value}")]
    InvalidEnumValue {
        /// Type being converted to
        type_name: &'static str,
        /// Invalid value received
        value: String,
    },
}

/// Result type alias for Hyprland operations.
pub type Result<T> = std::result::Result<T, Error>;
