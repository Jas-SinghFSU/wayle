use std::sync::Arc;

use derive_more::Debug;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{instrument, warn};
use wayle_common::Property;
use zbus::Connection;

use crate::{
    builder::NotificationServiceBuilder, core::notification::Notification, error::Error,
    events::NotificationEvent, persistence::NotificationStore, popup_timer::PopupTimerManager,
    types::ClosedReason,
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
    /// Glob patterns for blocking notifications by app name.
    pub blocklist: Property<Vec<String>>,
    #[debug(skip)]
    pub(crate) popup_timers: Arc<PopupTimerManager>,
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

    /// Dismisses all notifications and emits `NotificationClosed` for each.
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
    pub fn set_dnd(&self, dnd: bool) {
        self.dnd.set(dnd)
    }

    /// Sets the duration for how long popup notifications are displayed.
    pub fn set_popup_duration(&self, duration: u32) {
        self.popup_duration.set(duration)
    }

    /// Replaces the blocklist patterns.
    pub fn set_blocklist(&self, patterns: Vec<String>) {
        self.blocklist.set(patterns)
    }

    /// Removes a popup from the visible list without affecting notification history.
    ///
    /// Cancels any running popup timer for this ID.
    pub fn dismiss_popup(&self, id: u32) {
        self.popup_timers.cancel(id);

        let mut list = self.popups.get();
        list.retain(|popup| popup.id != id);
        self.popups.set(list);
    }

    /// Pauses the popup countdown timer.
    pub fn inhibit_popup(&self, id: u32) {
        self.popup_timers.pause(id);
    }

    /// Resumes the popup countdown timer after a pause.
    pub fn release_popup(&self, id: u32) {
        self.popup_timers.resume(id);
    }
}

impl Drop for NotificationService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
