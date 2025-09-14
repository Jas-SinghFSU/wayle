use tokio_util::sync::CancellationToken;
use zbus::Connection;

/// Service for handling desktop notifications.
pub struct NotificationService {
    pub(crate) zbus_connection: Connection,
    pub(crate) cancellation_token: CancellationToken,
}

impl NotificationService {
    /// Creates a new notification service instance.
    pub async fn new() -> Self {
        todo!()
    }
}
