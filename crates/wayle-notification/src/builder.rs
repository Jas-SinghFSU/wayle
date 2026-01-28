use std::sync::{Arc, atomic::AtomicU32};

use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;
use zbus::{Connection, object_server::Interface};

use crate::{
    core::{notification::Notification, types::NotificationProps},
    daemon::NotificationDaemon,
    error::Error,
    persistence::{NotificationStore, StoredNotification},
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
    /// Establishes a D-Bus connection, registers the notification daemon,
    /// restores persisted notifications, and starts monitoring for events.
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

        let store = init_store();
        let stored_notifications =
            load_stored_notifications(&store, self.remove_expired.get(), &connection);
        let max_id = stored_notifications.iter().map(|n| n.id).max().unwrap_or(0);

        let freedesktop_daemon = NotificationDaemon {
            counter: AtomicU32::new(max_id + 1),
            zbus_connection: connection.clone(),
            notif_tx: notif_tx.clone(),
        };

        register_dbus_object(&connection, SERVICE_PATH, freedesktop_daemon).await?;
        register_dbus_name(&connection, SERVICE_NAME).await?;
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
            register_dbus_object(&connection, WAYLE_SERVICE_PATH, wayle_daemon).await?;
            register_dbus_name(&connection, WAYLE_SERVICE_NAME).await?;
            info!("Wayle notification extensions registered at {WAYLE_SERVICE_NAME}");
        }

        Ok(service)
    }
}

fn init_store() -> Option<NotificationStore> {
    match NotificationStore::new() {
        Ok(store) => {
            info!("Notification persistence enabled");
            Some(store)
        }
        Err(e) => {
            error!(error = %e, "cannot initialize notification store");
            error!("notifications will not persist across restarts");
            None
        }
    }
}

fn load_stored_notifications(
    store: &Option<NotificationStore>,
    remove_expired: bool,
    connection: &Connection,
) -> Vec<Arc<Notification>> {
    store
        .as_ref()
        .and_then(|s| s.load_all(remove_expired).ok())
        .map(|stored| {
            stored
                .into_iter()
                .map(|n| stored_to_notification(n, connection.clone()))
                .collect()
        })
        .unwrap_or_default()
}

fn stored_to_notification(stored: StoredNotification, connection: Connection) -> Arc<Notification> {
    Arc::new(Notification::new(
        NotificationProps {
            id: stored.id,
            app_name: stored.app_name.unwrap_or_default(),
            replaces_id: stored.replaces_id.unwrap_or(0),
            app_icon: stored.app_icon.unwrap_or_default(),
            summary: stored.summary,
            body: stored.body.unwrap_or_default(),
            actions: stored.actions,
            hints: stored.hints,
            expire_timeout: stored.expire_timeout.unwrap_or(0) as i32,
            timestamp: DateTime::<Utc>::from_timestamp_millis(stored.timestamp)
                .unwrap_or_else(Utc::now),
        },
        connection,
    ))
}

async fn register_dbus_object<T: Interface>(
    connection: &Connection,
    path: &str,
    object: T,
) -> Result<(), Error> {
    connection
        .object_server()
        .at(path, object)
        .await
        .map_err(|err| {
            Error::ServiceInitializationFailed(format!(
                "cannot register D-Bus object at '{path}': {err}"
            ))
        })?;
    Ok(())
}

async fn register_dbus_name(connection: &Connection, name: &str) -> Result<(), Error> {
    connection.request_name(name).await.map_err(|err| {
        Error::ServiceInitializationFailed(format!("cannot acquire D-Bus name '{name}': {err}"))
    })
}
