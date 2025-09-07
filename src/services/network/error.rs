use zbus::zvariant::OwnedObjectPath;

use super::types::connectivity::ConnectionType;

/// Network service errors
#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed (used for top-level service startup)
    #[error("Failed to initialize network service: {0}")]
    ServiceInitializationFailed(String),

    /// Object not found at the specified D-Bus path
    #[error("Object not found at path: {0}")]
    ObjectNotFound(OwnedObjectPath),

    /// Object exists but is of wrong type
    #[error("Object at {object_path} is wrong type: expected {expected}, got {actual}")]
    WrongObjectType {
        /// DBus object path that has wrong type.
        object_path: OwnedObjectPath,
        /// Expected object type.
        expected: String,
        /// Actual object type found.
        actual: String,
    },

    /// Failed to create or fetch an object
    #[error("Failed to create {object_type} at {object_path}: {reason}")]
    ObjectCreationFailed {
        /// Type of object that failed to create.
        object_type: String,
        /// DBus path where creation failed.
        object_path: OwnedObjectPath,
        /// Reason for the failure.
        reason: String,
    },

    /// Device not found by identifier
    #[error("Device {0} not found")]
    DeviceNotFound(String),

    /// Access point not found by SSID
    #[error("Access point {ssid} not found")]
    AccessPointNotFound {
        /// SSID of the access point that was not found.
        ssid: String,
    },

    /// Connection activation failed
    #[error("Failed to activate connection: {0}")]
    ActivationFailed(String),

    /// No active connection of specified type
    #[error("No active {0:?} connection")]
    NoActiveConnection(ConnectionType),

    /// Network operation failed
    #[error("Network operation failed: {operation} - {reason}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The reason the operation failed
        reason: String,
    },

    /// Data conversion or parsing failed
    #[error("Failed to parse {data_type}: {reason}")]
    DataConversionFailed {
        /// Type of data that failed to convert.
        data_type: String,
        /// Reason for conversion failure.
        reason: String,
    },
}
