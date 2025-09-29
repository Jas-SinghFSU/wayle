/// Notification service errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// D-Bus communication error
    #[error("D-Bus operation failed: {0:#?}")]
    DbusError(#[from] zbus::Error),

    /// Service initialization failed
    #[error("Failed to initialize notification service: {0:#?}")]
    ServiceInitializationFailed(String),

    /// Failed to claim the notification service name on D-Bus
    #[error("Failed to claim org.freedesktop.Notifications: {0:#?}")]
    NameClaimFailed(String),

    /// Database operation failed
    #[error("Database operation failed: {0:#?}")]
    DatabaseError(String),

    /// Notification not found
    #[error("Notification with ID {0} not found")]
    NotificationNotFound(u32),

    /// Invalid notification data
    #[error("Invalid notification data: {0}")]
    InvalidNotificationData(String),

    /// Operation failed
    #[error("Notification operation failed: {operation} - {reason}")]
    OperationFailed {
        /// The operation that failed
        operation: &'static str,
        /// The reason the operation failed
        reason: String,
    },
}
