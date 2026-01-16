use std::sync::Arc;

use tracing::instrument;
use zbus::{fdo, interface};

use crate::{events::NotificationEvent, service::NotificationService, types::ClosedReason};

#[derive(Debug)]
pub(crate) struct WayleDaemon {
    pub service: Arc<NotificationService>,
}

#[interface(name = "com.wayle.Notifications1")]
impl WayleDaemon {
    /// Dismisses all notifications.
    #[instrument(skip(self))]
    pub async fn dismiss_all(&self) -> fdo::Result<()> {
        self.service
            .dismiss_all()
            .await
            .map_err(|e| fdo::Error::Failed(e.to_string()))
    }

    /// Dismisses a specific notification by ID.
    #[instrument(skip(self), fields(id = id))]
    pub async fn dismiss(&self, id: u32) -> fdo::Result<()> {
        self.service
            .notif_tx
            .send(NotificationEvent::Remove(id, ClosedReason::DismissedByUser))
            .map_err(|e| fdo::Error::Failed(e.to_string()))?;
        Ok(())
    }

    /// Sets Do Not Disturb mode.
    ///
    /// When enabled, new notifications won't appear as popups.
    #[instrument(skip(self), fields(enabled = enabled))]
    pub async fn set_dnd(&self, enabled: bool) -> fdo::Result<()> {
        self.service.set_dnd(enabled).await;
        Ok(())
    }

    /// Toggles Do Not Disturb mode.
    #[instrument(skip(self))]
    pub async fn toggle_dnd(&self) -> fdo::Result<()> {
        let current = self.service.dnd.get();
        self.service.set_dnd(!current).await;
        Ok(())
    }

    /// Sets the popup display duration in milliseconds.
    #[instrument(skip(self), fields(duration_ms = duration_ms))]
    pub async fn set_popup_duration(&self, duration_ms: u32) -> fdo::Result<()> {
        self.service.set_popup_duration(duration_ms).await;
        Ok(())
    }

    /// Lists all notifications.
    ///
    /// Returns a list of tuples: (id, app_name, summary, body).
    #[instrument(skip(self))]
    pub async fn list(&self) -> Vec<(u32, String, String, String)> {
        self.service
            .notifications
            .get()
            .iter()
            .map(|n| {
                (
                    n.id,
                    n.app_name.get().unwrap_or_default(),
                    n.summary.get(),
                    n.body.get().unwrap_or_default(),
                )
            })
            .collect()
    }

    /// Do Not Disturb status.
    #[zbus(property)]
    pub async fn dnd(&self) -> bool {
        self.service.dnd.get()
    }

    /// Popup display duration in milliseconds.
    #[zbus(property)]
    pub async fn popup_duration(&self) -> u32 {
        self.service.popup_duration.get()
    }

    /// Number of notifications.
    #[zbus(property)]
    pub async fn count(&self) -> u32 {
        self.service.notifications.get().len() as u32
    }

    /// Number of active popups.
    #[zbus(property)]
    pub async fn popup_count(&self) -> u32 {
        self.service.popups.get().len() as u32
    }
}
