use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use chrono::Utc;
use derive_more::Debug;
use tokio::sync::broadcast;
use tracing::{debug, instrument};
use wayle_common::{Property, glob};
use zbus::{
    Connection, fdo,
    zvariant::{OwnedValue, Str},
};

use crate::{
    core::{
        notification::Notification,
        types::{IMAGE_DATA_KEYS, ImageData, NotificationHints, NotificationProps},
    },
    events::NotificationEvent,
    image_cache,
    types::{Capabilities, ClosedReason, Name, SpecVersion, Vendor, Version},
};

#[derive(Debug)]
pub(crate) struct NotificationDaemon {
    pub counter: AtomicU32,
    #[debug(skip)]
    pub zbus_connection: Connection,
    #[debug(skip)]
    pub notif_tx: broadcast::Sender<NotificationEvent>,
    #[debug(skip)]
    pub blocklist: Property<Vec<String>>,
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

        let blocked = self
            .blocklist
            .get()
            .iter()
            .any(|pattern| glob::matches(pattern, &app_name));

        if blocked {
            debug!(app = %app_name, "notification blocked by blocklist");
            return Ok(id);
        }

        let hints = replace_image_data_with_cached_png(hints);

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
            self.notif_tx.clone(),
        );

        let notif_id = notif.id;
        let _ = self.notif_tx.send(NotificationEvent::Add(Box::new(notif)));

        Ok(notif_id)
    }

    #[instrument(skip(self), fields(notification_id = %id))]
    pub async fn close_notification(&self, id: u32) -> fdo::Result<()> {
        let _ = self
            .notif_tx
            .send(NotificationEvent::Remove(id, ClosedReason::Closed));
        Ok(())
    }

    pub async fn get_capabilities(&self) -> Vec<String> {
        vec![
            Capabilities::Body.to_string(),
            Capabilities::BodyMarkup.to_string(),
            Capabilities::Actions.to_string(),
            Capabilities::IconStatic.to_string(),
            Capabilities::Persistence.to_string(),
        ]
    }

    pub async fn get_server_information(&self) -> (Name, Vendor, Version, SpecVersion) {
        let name = String::from("wayle");
        let vendor = String::from("jaskir");
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let spec_version = String::from("1.2");

        (name, vendor, version, spec_version)
    }
}

fn replace_image_data_with_cached_png(mut hints: NotificationHints) -> NotificationHints {
    let Some(image) = ImageData::from_hints(&hints) else {
        return hints;
    };

    if let Some(cached_path) = image_cache::cache_image(&image) {
        hints.insert(
            String::from("image-path"),
            OwnedValue::from(Str::from(cached_path)),
        );
    }

    for key in IMAGE_DATA_KEYS {
        hints.remove(key);
    }

    hints
}
