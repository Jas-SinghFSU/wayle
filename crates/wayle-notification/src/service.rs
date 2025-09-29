use std::sync::{Arc, atomic::AtomicU32};

use chrono::{DateTime, Utc};
use derive_more::Debug;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, instrument, warn};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    core::{notification::Notification, types::NotificationProps},
    daemon::NotificationDaemon,
    error::Error,
    events::NotificationEvent,
    persistence::NotificationStore,
    types::{
        ClosedReason,
        dbus::{SERVICE_NAME, SERVICE_PATH},
    },
};

/// Service for handling desktop notifications.
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

    /// The list of all notifications that have been received.
    pub notifications: Property<Vec<Arc<Notification>>>,
    /// The list of notifications currently shown as popups.
    pub popups: Property<Vec<Arc<Notification>>>,
    /// Duration in milliseconds for how long popups should be shown.
    pub popup_duration: Property<u32>,
    /// Do Not Disturb mode - when enabled, popups are suppressed.
    pub dnd: Property<bool>,
    /// Whether to automatically remove expired notifications
    pub remove_expired: Property<bool>,
}

impl NotificationService {
    /// Creates a new notification service instance.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails or service registration fails.
    #[instrument(name = "NotificationService::new", err)]
    pub async fn new() -> Result<Self, Error> {
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
            if let Err(e) = self.notif_tx.send(NotificationEvent::Remove(
                notif.id,
                ClosedReason::DismissedByUser,
            )) {
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

/// Builder for configuring and creating a NotificationService instance.
///
/// Allows customization of popup duration, do-not-disturb mode, and
/// automatic removal of expired notifications.
#[derive(Debug)]
pub struct NotificationServiceBuilder {
    popup_duration: Property<u32>,
    dnd: Property<bool>,
    remove_expired: Property<bool>,
    register_daemon: bool,
}

impl Default for NotificationServiceBuilder {
    fn default() -> Self {
        Self {
            popup_duration: Property::new(5000),
            dnd: Property::new(false),
            remove_expired: Property::new(true),
            register_daemon: true,
        }
    }
}

impl NotificationServiceBuilder {
    /// Creates a new NotificationServiceBuilder with default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the duration in milliseconds for how long popups should be displayed.
    pub fn popup_duration(self, duration: u32) -> Self {
        self.popup_duration.set(duration);
        self
    }

    /// Configures the Do Not Disturb mode.
    ///
    /// When enabled, new notifications won't appear as popups but will still
    /// be added to the notification list.
    pub fn dnd(self, dnd: bool) -> Self {
        self.dnd.set(dnd);
        self
    }

    /// Sets whether to automatically remove expired notifications.
    pub fn remove_expired(self, remove: bool) -> Self {
        self.remove_expired.set(remove);
        self
    }

    /// Sets whether to register as the D-Bus notification daemon.
    /// Set to false when creating a client that shouldn't own the service name.
    pub fn register_daemon(mut self, register: bool) -> Self {
        self.register_daemon = register;
        self
    }

    /// Builds and initializes the NotificationService.
    ///
    /// This will establish a D-Bus connection, register the notification daemon,
    /// restore persisted notifications, and start monitoring for events.
    ///
    /// # Errors
    /// Returns error if D-Bus connection fails, service registration fails,
    /// or monitoring cannot be started.
    pub async fn build(self) -> Result<NotificationService, Error> {
        let connection = Connection::session().await.map_err(|err| {
            Error::ServiceInitializationFailed(format!("D-Bus connection failed: {err}"))
        })?;
        let (notif_tx, _) = broadcast::channel(10000);
        let cancellation_token = CancellationToken::new();

        let store = match NotificationStore::new() {
            Ok(store) => {
                info!("Notification persistence enabled");
                Some(store)
            }
            Err(e) => {
                error!("Failed to initialize notification store: {}", e);
                error!("Notifications will not persist across restarts");
                None
            }
        };

        let stored_notifications: Vec<Arc<Notification>> = store
            .as_ref()
            .and_then(|s| s.load_all(self.remove_expired.get()).ok())
            .map(|stored| {
                stored
                    .into_iter()
                    .map(|n| {
                        Arc::new(Notification::new(
                            NotificationProps {
                                id: n.id,
                                app_name: n.app_name.unwrap_or_default(),
                                replaces_id: n.replaces_id.unwrap_or(0),
                                app_icon: n.app_icon.unwrap_or_default(),
                                summary: n.summary,
                                body: n.body.unwrap_or_default(),
                                actions: n.actions,
                                hints: n.hints,
                                expire_timeout: n.expire_timeout.unwrap_or(0) as i32,
                                timestamp: DateTime::<Utc>::from_timestamp_millis(n.timestamp)
                                    .unwrap_or_else(Utc::now),
                            },
                            connection.clone(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let max_id = stored_notifications.iter().map(|n| n.id).max().unwrap_or(0);
        let counter = AtomicU32::new(max_id + 1);
        let daemon = NotificationDaemon {
            counter,
            zbus_connection: connection.clone(),
            notif_tx: notif_tx.clone(),
        };

        if self.register_daemon {
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
        }

        let service = NotificationService {
            cancellation_token,
            notif_tx,
            store,
            connection,
            notifications: Property::new(stored_notifications),
            popups: Property::new(vec![]),
            popup_duration: self.popup_duration,
            dnd: self.dnd,
            remove_expired: self.remove_expired,
        };

        service.start_monitoring().await?;

        Ok(service)
    }
}

impl Drop for NotificationService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
