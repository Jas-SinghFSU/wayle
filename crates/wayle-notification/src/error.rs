/// Notification service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed
    #[error("cannot initialize notification service: {0}")]
    ServiceInitializationFailed(String),

    /// Cannot claim the notification service name on D-Bus
    #[error("cannot claim org.freedesktop.Notifications: {0}")]
    NameClaimFailed(String),

    /// Database operation failed
    #[error("Database operation failed: {0}")]
    DatabaseError(String),

    /// Notification not found
    #[error("Notification with ID {0} not found")]
    NotificationNotFound(u32),

    /// Invalid notification data
    #[error("Invalid notification data: {0}")]
    InvalidNotificationData(String),

    /// Operation failed
    #[error("cannot {operation}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The underlying error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
