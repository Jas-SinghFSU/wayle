use std::sync::Arc;

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use zbus::Connection;

use super::{
    core::notification::Notification,
    daemon::NotificationDaemon,
    error::Error,
    events::NotificationEvent,
    types::dbus::{SERVICE_NAME, SERVICE_PATH},
};
use crate::services::{common::Property, traits::ServiceMonitoring};

/// Service for handling desktop notifications.
pub struct NotificationService {
    pub(crate) cancellation_token: CancellationToken,
    pub(crate) notif_tx: broadcast::Sender<NotificationEvent>,

    /// The list of all notifications that have been received.
    pub notifications: Property<Vec<Arc<Notification>>>,
    /// The list of notifications currently shown as popups.
    pub popups: Property<Vec<Arc<Notification>>>,
    /// Duration in milliseconds for how long popups should be shown.
    pub popup_duration: Property<u32>,
    /// Do Not Disturb mode - when enabled, popups are suppressed.
    pub dnd: Property<bool>,
}

impl NotificationService {
    /// Creates a new notification service instance.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service registration fails.
    pub async fn new() -> Result<Self, Error> {
        let connection = Connection::session().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;
        let (notif_tx, _) = broadcast::channel(100);
        let cancellation_token = CancellationToken::new();

        let daemon = NotificationDaemon {
            zbus_connection: connection.clone(),
            notif_tx: notif_tx.clone(),
        };

        connection
            .object_server()
            .at(SERVICE_PATH, daemon)
            .await
            .map_err(|err| {
                Error::ServiceInitializationFailed(format!("Failed to register daemon: {err}"))
            })?;

        connection.request_name(SERVICE_NAME).await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("Failed to acquire name: {err}"))
        })?;

        let service = Self {
            cancellation_token,
            notif_tx,
            notifications: Property::new(vec![]),
            popups: Property::new(vec![]),
            popup_duration: Property::new(5000),
            dnd: Property::new(false),
        };

        service.start_monitoring().await?;

        Ok(service)
    }

    /// Dismisses all notifications currently in the service.
    ///
    /// This sends a remove event for each notification, which will trigger
    /// the NotificationClosed signal with DismissedByUser reason for each.
    ///
    /// # Errors
    /// Returns error if the event channel is closed.
    pub async fn dismiss_all(&self) -> Result<(), Error> {
        let notifications = self.notifications.get();

        for notif in notifications.iter() {
            if let Err(e) = self.notif_tx.send(NotificationEvent::Remove(notif.id)) {
                warn!(
                    "Failed to dismiss notification with id '{}': {}",
                    notif.id, e
                );
            }
        }

        Ok(())
    }

    /// Sets the Do Not Disturb mode.
    ///
    /// When enabled, new notifications will not appear as popups but will
    /// still be added to the notification list.
    pub async fn set_dnd(&self, dnd: bool) {
        self.dnd.set(dnd)
    }

    /// Sets the duration for how long popup notifications are displayed.
    ///
    /// The duration is specified in milliseconds.
    pub async fn set_popup_duration(&self, duration: u32) {
        self.popup_duration.set(duration)
    }
}

impl Drop for NotificationService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
