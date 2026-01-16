use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{instrument, warn};
use wayle_common::Property;
use zbus::Connection;

use crate::{
    builder::NotificationServiceBuilder, core::notification::Notification, error::Error,
    events::NotificationEvent, persistence::NotificationStore, types::ClosedReason,
};

/// Desktop notification service. See [crate-level docs](crate) for usage.
#[derive(Debug)]
pub struct NotificationService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) notif_tx: broadcast::Sender<NotificationEvent>,
    #[debug(skip)]
    pub(crate) store: Option<NotificationStore>,
    #[debug(skip)]
    pub(crate) connection: Connection,

    /// All received notifications.
    pub notifications: Property<Vec<Arc<Notification>>>,
    /// Currently visible popups.
    pub popups: Property<Vec<Arc<Notification>>>,
    /// Popup display duration in milliseconds.
    pub popup_duration: Property<u32>,
    /// Do Not Disturb mode; suppresses popups when true.
    pub dnd: Property<bool>,
    /// Auto-remove expired notifications.
    pub remove_expired: Property<bool>,
}

impl NotificationService {
    /// Creates a new notification service instance.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service registration fails.
    #[instrument(name = "NotificationService::new", err)]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring a NotificationService.
    pub fn builder() -> NotificationServiceBuilder {
        NotificationServiceBuilder::new()
    }

    /// Dismisses all notifications currently in the service.
    ///
    /// This sends Remove events for each notification. The monitoring task
    /// handles the actual removal from memory, database, and emits the
    /// NotificationClosed signals.
    ///
    /// # Errors
    /// Returns error if the event channel is closed.
    #[instrument(skip(self), err)]
    pub async fn dismiss_all(&self) -> Result<(), Error> {
        let notifications = self.notifications.get();

        for notif in notifications.iter() {
            if let Err(error) = self.notif_tx.send(NotificationEvent::Remove(
                notif.id,
                ClosedReason::DismissedByUser,
            )) {
                warn!(error = %error, id = notif.id, "cannot dismiss notification");
            }
        }

        Ok(())
    }

    /// Sets the Do Not Disturb mode.
    ///
    /// When enabled, new notifications will not appear as popups but will
    /// still be added to the notification list.
    #[instrument(skip(self), fields(dnd = %dnd))]
    pub async fn set_dnd(&self, dnd: bool) {
        self.dnd.set(dnd)
    }

    /// Sets the duration for how long popup notifications are displayed.
    ///
    /// The duration is specified in milliseconds.
    #[instrument(skip(self), fields(duration_ms = %duration))]
    pub async fn set_popup_duration(&self, duration: u32) {
        self.popup_duration.set(duration)
    }
}

impl Drop for NotificationService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
