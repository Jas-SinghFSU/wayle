use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use chrono::Utc;
use derive_more::Debug;
use tokio::sync::broadcast;
use tracing::instrument;
use zbus::{Connection, fdo, zvariant::OwnedValue};

use super::{
    core::{notification::Notification, types::NotificationProps},
    events::NotificationEvent,
    types::{ClosedReason, Name, SpecVersion, Vendor, Version},
};
use crate::services::notification::types::Capabilities;

#[derive(Debug)]
pub(crate) struct NotificationDaemon {
    pub counter: AtomicU32,
    #[debug(skip)]
    pub zbus_connection: Connection,
    #[debug(skip)]
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

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(expire_timeout as u64)).await;
                let _ = tx.send(NotificationEvent::Remove(notif_id, ClosedReason::Expired));
            });
        }

        Ok(notif_id)
    }

    #[instrument(skip(self), fields(notification_id = %id))]
    pub async fn close_notification(&self, id: u32) -> fdo::Result<()> {
        let _ = self
            .notif_tx
            .send(NotificationEvent::Remove(id, ClosedReason::Closed));
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_capabilities(&self) -> Vec<String> {
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
