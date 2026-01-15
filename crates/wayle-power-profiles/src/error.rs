/// Power profiles service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0}")]
    DbusError(
        #[from]
        #[source]
        zbus::Error,
    ),

    /// Service initialization failed
    #[error("cannot initialize power profiles service: {0}")]
    ServiceInitializationFailed(String),

    /// Invalid field type in D-Bus data
    #[error("Invalid field type for {field}: expected {expected}")]
    InvalidFieldType {
        /// Field name that had invalid type
        field: String,
        /// Expected type description
        expected: String,
    },

    /// Monitoring cannot start without a cancellation token
    #[error("cannot start monitoring: cancellation token was not provided")]
    MissingCancellationToken,
}
