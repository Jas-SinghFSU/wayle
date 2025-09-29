/// Power profiles service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0:#?}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed
    #[error("Failed to initialize power profiles service: {0:#?}")]
    ServiceInitializationFailed(String),

    /// Invalid field type in D-Bus data
    #[error("Invalid field type for {field}: expected {expected}")]
    InvalidFieldType {
        /// Field name that had invalid type
        field: String,
        /// Expected type description
        expected: String,
    },

    /// PowerProfiles operation failed
    #[error("PowerProfiles operation failed: {operation} - {reason}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The reason the operation failed
        reason: String,
    },
}
