use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{instrument, warn};
use zbus::{Connection, fdo, zvariant::OwnedValue};

use super::{
    core::{notification::Notification, types::NotificationProps},
    events::NotificationEvent,
    types::{
        ClosedReason, Name, Signal, SpecVersion, Vendor, Version,
        dbus::{SERVICE_INTERFACE, SERVICE_PATH},
    },
};

pub(crate) struct NotificationDaemon {
    pub counter: AtomicU32,
    pub zbus_connection: Connection,
    pub notif_tx: broadcast::Sender<NotificationEvent>,
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl NotificationDaemon {
    #[allow(clippy::too_many_arguments)]
    #[instrument(
        skip(self, actions, hints),
        fields(
            app = %app_name,
            replaces = %replaces_id,
            timeout = %expire_timeout
        )
    )]
    pub async fn notify(
        &self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, OwnedValue>,
        expire_timeout: i32,
    ) -> fdo::Result<u32> {
        let id = if replaces_id > 0 {
            replaces_id
        } else {
            self.counter.fetch_add(1, Ordering::Relaxed)
        };

        let notif = Notification::new(
            NotificationProps {
                id,
                app_name,
                replaces_id,
                app_icon,
                summary,
                body,
                actions,
                hints,
                expire_timeout,
                timestamp: Utc::now(),
            },
            self.zbus_connection.clone(),
        );

        let notif_id = notif.id;
        let _ = self.notif_tx.send(NotificationEvent::Add(Box::new(notif)));

        if expire_timeout > 0 {
            let tx = self.notif_tx.clone();
            let connection = self.zbus_connection.clone();

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(expire_timeout as u64)).await;
                let _ = tx.send(NotificationEvent::Remove(notif_id));

                if let Err(e) = connection
                    .emit_signal(
                        None::<()>,
                        SERVICE_PATH,
                        SERVICE_INTERFACE,
                        Signal::NotificationClosed.as_str(),
                        &(notif_id, ClosedReason::Expired as u32),
                    )
                    .await
                {
                    warn!(
                        "Failed to emit NotificationClosed signal for expired notification {}: {}",
                        notif_id, e
                    );
                }
            });
        }

        Ok(notif_id)
    }

    #[instrument(skip(self), fields(notification_id = %id))]
    pub async fn close_notification(&self, id: u32) -> fdo::Result<()> {
        let _ = self.notif_tx.send(NotificationEvent::Remove(id));

        if let Err(e) = self
            .zbus_connection
            .emit_signal(
                None::<()>,
                SERVICE_PATH,
                SERVICE_INTERFACE,
                Signal::NotificationClosed.as_str(),
                &(id, ClosedReason::Closed as u32),
            )
            .await
        {
            warn!(
                "Failed to emit NotificationClosed signal for CloseNotification({}): {}",
                id, e
            );
        }

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_capabilities(&self) -> Vec<String> {
        use super::types::Capabilities;

        vec![
            Capabilities::Body.to_string(),
            Capabilities::BodyMarkup.to_string(),
            Capabilities::Actions.to_string(),
            Capabilities::IconStatic.to_string(),
            Capabilities::Persistence.to_string(),
        ]
    }

    #[instrument(skip(self))]
    pub async fn get_server_information(&self) -> (Name, Vendor, Version, SpecVersion) {
        let name = String::from("wayle");
        let vendor = String::from("jaskir");
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let spec_version = String::from("1.2");

        (name, vendor, version, spec_version)
    }
}
