//! Builder for configuring a NotificationService.

use std::sync::{Arc, atomic::AtomicU32};

use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::Connection;

use crate::{
    core::{notification::Notification, types::NotificationProps},
    daemon::NotificationDaemon,
    error::Error,
    persistence::NotificationStore,
    service::NotificationService,
    types::dbus::{SERVICE_NAME, SERVICE_PATH, WAYLE_SERVICE_NAME, WAYLE_SERVICE_PATH},
    wayle_daemon::WayleDaemon,
};

/// Builder for configuring and creating a NotificationService instance.
///
/// Allows customization of popup duration, do-not-disturb mode, and
/// automatic removal of expired notifications.
#[derive(Debug)]
pub struct NotificationServiceBuilder {
    popup_duration: Property<u32>,
    dnd: Property<bool>,
    remove_expired: Property<bool>,
    register_wayle_daemon: bool,
}

impl Default for NotificationServiceBuilder {
    fn default() -> Self {
        Self {
            popup_duration: Property::new(5000),
            dnd: Property::new(false),
            remove_expired: Property::new(true),
            register_wayle_daemon: false,
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

    /// Enables the Wayle D-Bus daemon for CLI control.
    ///
    /// When enabled, the service registers at `com.wayle.Notifications1`,
    /// allowing CLI tools to control notifications (dismiss, toggle DND, etc.).
    pub fn with_daemon(mut self) -> Self {
        self.register_wayle_daemon = true;
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
    pub async fn build(self) -> Result<Arc<NotificationService>, Error> {
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
                error!(error = %e, "cannot initialize notification store");
                error!("notifications will not persist across restarts");
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
        let freedesktop_daemon = NotificationDaemon {
            counter,
            zbus_connection: connection.clone(),
            notif_tx: notif_tx.clone(),
        };

        connection
            .object_server()
            .at(SERVICE_PATH, freedesktop_daemon)
            .await
            .map_err(|err| {
                Error::ServiceInitializationFailed(format!(
                    "cannot register D-Bus object at '{SERVICE_PATH}': {err}"
                ))
            })?;

        connection.request_name(SERVICE_NAME).await.map_err(|err| {
            Error::ServiceInitializationFailed(format!(
                "cannot acquire D-Bus name '{SERVICE_NAME}': {err}"
            ))
        })?;

        info!("Notification daemon registered at {SERVICE_NAME}");

        let service = Arc::new(NotificationService {
            cancellation_token,
            notif_tx,
            store,
            connection: connection.clone(),
            notifications: Property::new(stored_notifications),
            popups: Property::new(vec![]),
            popup_duration: self.popup_duration,
            dnd: self.dnd,
            remove_expired: self.remove_expired,
        });

        service.start_monitoring().await?;

        if self.register_wayle_daemon {
            let wayle_daemon = WayleDaemon {
                service: Arc::clone(&service),
            };

            connection
                .object_server()
                .at(WAYLE_SERVICE_PATH, wayle_daemon)
                .await
                .map_err(|err| {
                    Error::ServiceInitializationFailed(format!(
                        "cannot register D-Bus object at '{WAYLE_SERVICE_PATH}': {err}"
                    ))
                })?;

            connection
                .request_name(WAYLE_SERVICE_NAME)
                .await
                .map_err(|err| {
                    Error::ServiceInitializationFailed(format!(
                        "cannot acquire D-Bus name '{WAYLE_SERVICE_NAME}': {err}"
                    ))
                })?;

            info!("Wayle notification extensions registered at {WAYLE_SERVICE_NAME}");
        }

        Ok(service)
    }
}
