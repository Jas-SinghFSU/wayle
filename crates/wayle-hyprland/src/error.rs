use std::{io, string::FromUtf8Error};

use tokio::sync::broadcast;

use crate::{Address, HyprlandEvent, WorkspaceId};

/// Hyprland service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Cannot initialize Hyprland service
    #[error("cannot initialize hyprland service: {0}")]
    ServiceInitializationFailed(String),

    /// Cannot connect to IPC socket
    #[error("cannot connect to hyprland IPC {socket_type} socket")]
    IpcConnectionFailed {
        /// Type of socket (command or event)
        socket_type: &'static str,
        /// Underlying I/O error
        #[source]
        source: io::Error,
    },

    /// Socket I/O operation error
    #[error("IPC socket I/O error")]
    SocketIoError(
        #[from]
        #[source]
        io::Error,
    ),

    /// Cannot parse JSON response from Hyprland
    #[error("cannot parse hyprland response")]
    JsonParseError(
        #[from]
        #[source]
        serde_json::Error,
    ),

    /// Cannot parse event field value
    #[error("cannot parse {field} in event: expected {expected}, got {value:?}")]
    EventParseError {
        /// Raw event data that failed to parse
        event_data: String,
        /// Field that failed to parse
        field: &'static str,
        /// Expected format or value
        expected: &'static str,
        /// Actual value received
        value: String,
    },

    /// Workspace not found
    #[error("workspace {0} not found")]
    WorkspaceNotFound(WorkspaceId),

    /// Monitor not found
    #[error("monitor {0} not found")]
    MonitorNotFound(String),

    /// Client/Window not found
    #[error("client {0} not found")]
    ClientNotFound(Address),

    /// Layer not found
    #[error("layer {0} not found")]
    LayerNotFound(Address),

    /// Invalid Hyprland instance signature
    #[error("invalid hyprland instance signature: {0}")]
    InvalidInstanceSignature(String),

    /// Hyprland is not running or not accessible
    #[error("hyprland is not running or HYPRLAND_INSTANCE_SIGNATURE is not set")]
    HyprlandNotRunning,

    /// Operation not supported by current Hyprland version
    #[error("operation not supported: {0}")]
    OperationNotSupported(String),

    /// Cannot decode IPC response as UTF-8
    #[error("cannot decode IPC response as UTF-8")]
    ResponseDecodeError(#[source] FromUtf8Error),

    /// Cannot start monitoring without required resources
    #[error("cannot start monitoring for {resource_type} '{resource_id}': {missing_resource}")]
    MonitoringSetupError {
        /// Type of resource (workspace, client, monitor)
        resource_type: &'static str,
        /// Identifier of the resource
        resource_id: String,
        /// What resource is missing
        missing_resource: &'static str,
    },

    /// Cannot transmit hyprland event
    #[error("cannot transmit hyprland event")]
    HyprlandEventTransmitError(#[from] broadcast::error::SendError<HyprlandEvent>),

    /// Cannot transmit internal service notification
    #[error("cannot transmit internal service notification: {0}")]
    InternalEventTransmitError(String),

    /// Invalid value for enum conversion
    #[error("invalid value for {type_name}: {value}")]
    InvalidEnumValue {
        /// Type being converted to
        type_name: &'static str,
        /// Invalid value received
        value: String,
    },
}

/// Result type alias for Hyprland operations.
pub type Result<T> = std::result::Result<T, Error>;
