use zbus::zvariant::OwnedObjectPath;

/// Network service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error.
    #[error("dbus operation failed: {0}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed (used for top-level service startup).
    #[error("cannot initialize network service: {0}")]
    ServiceInitializationFailed(String),

    /// Object not found at the specified D-Bus path.
    #[error("object not found at path: {0}")]
    ObjectNotFound(OwnedObjectPath),

    /// Object exists but is of wrong type.
    #[error("object at {object_path} is wrong type: expected {expected}, got {actual}")]
    WrongObjectType {
        /// DBus object path that has wrong type.
        object_path: OwnedObjectPath,
        /// Expected object type.
        expected: String,
        /// Actual object type found.
        actual: String,
    },

    /// Cannot create or fetch an object.
    #[error("cannot create {object_type} at {object_path}")]
    ObjectCreationFailed {
        /// Type of object that failed to create.
        object_type: String,
        /// DBus path where creation failed.
        object_path: OwnedObjectPath,
        /// Underlying error that caused the failure.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Network operation failed.
    #[error("cannot {operation}")]
    OperationFailed {
        /// The operation that failed.
        operation: &'static str,
        /// Underlying error that caused the failure.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Monitoring requires a cancellation token.
    #[error("cannot start monitoring: cancellation token not provided")]
    MissingCancellationToken,

    /// Data conversion or parsing failed.
    #[error("cannot parse {data_type}: {reason}")]
    DataConversionFailed {
        /// Type of data that failed to convert.
        data_type: String,
        /// Reason for conversion failure.
        reason: String,
    },
}
