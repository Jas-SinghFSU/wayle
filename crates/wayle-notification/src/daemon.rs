use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
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
        types::{BorrowedImageData, IncomingHints, NotificationHints, NotificationProps},
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
    #[debug(skip)]
    pub id_owners: Mutex<HashMap<u32, String>>,
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
    pub fn notify(
        &self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: IncomingHints<'_>,
        expire_timeout: i32,
    ) -> fdo::Result<u32> {
        let id = self.resolve_id(replaces_id, &app_name);

        let blocked = self
            .blocklist
            .get()
            .iter()
            .any(|pattern| glob::matches(pattern, &app_name));

        if blocked {
            debug!(app = %app_name, "notification blocked by blocklist");
            return Ok(id);
        }

        let hints = normalize_hints(hints);
        self.register_owner(id, &app_name);

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
        self.remove_owner(id);
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

impl NotificationDaemon {
    /// Only allows an app to reuse `replaces_id` values it owns.
    /// Assigns a new ID otherwise.
    fn resolve_id(&self, replaces_id: u32, app_name: &str) -> u32 {
        if replaces_id == 0 {
            let new_id = self.counter.fetch_add(1, Ordering::Relaxed);
            debug!(new_id, "assigned new notification id");
            return new_id;
        }

        let owned_by_caller = self
            .id_owners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&replaces_id)
            .is_some_and(|owner| owner == app_name);

        if owned_by_caller {
            debug!(replaces_id, "reusing replaces_id owned by same app");
            replaces_id
        } else {
            let new_id = self.counter.fetch_add(1, Ordering::Relaxed);
            debug!(
                replaces_id,
                new_id, "replaces_id belongs to different app, assigned new id"
            );
            new_id
        }
    }

    fn register_owner(&self, id: u32, app_name: &str) {
        let mut owners = self
            .id_owners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        owners.insert(id, app_name.to_owned());
    }

    fn remove_owner(&self, id: u32) {
        let mut owners = self
            .id_owners
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        owners.remove(&id);
    }
}

fn normalize_hints(hints: IncomingHints<'_>) -> NotificationHints {
    normalize_hints_with(hints, image_cache::cache_borrowed_image)
}

fn normalize_hints_with<F>(hints: IncomingHints<'_>, cache_image: F) -> NotificationHints
where
    F: FnOnce(BorrowedImageData<'_>) -> Option<String>,
{
    let cached_path = hints.image_data().and_then(cache_image);
    let mut normalized = hints.into_owned();

    if let Some(cached_path) = cached_path {
        normalized.insert(
            String::from("image-path"),
            OwnedValue::from(Str::from(cached_path)),
        );
    }

    normalized
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use zbus::zvariant::{LE, Value, serialized::Context, to_bytes};

    use super::*;

    #[test]
    fn normalize_hints_replaces_image_data_with_cached_path() {
        let pixels = [0u8, 1, 2, 3];
        let mut raw = HashMap::new();
        raw.insert("category", Value::new("im.received"));
        raw.insert(
            "image-data",
            Value::new((1i32, 1i32, 4i32, true, 8i32, 4i32, &pixels[..])),
        );
        let encoded = to_bytes(Context::new_dbus(LE, 0), &raw).expect("hints should encode");
        let (hints, _) = encoded
            .deserialize::<IncomingHints<'_>>()
            .expect("hints should decode");

        let normalized = normalize_hints_with(hints, |_| Some(String::from("/tmp/fake.png")));

        assert!(normalized.contains_key("category"));
        assert!(!normalized.contains_key("image-data"));
        assert_eq!(
            normalized
                .get("image-path")
                .and_then(|value| value.downcast_ref::<String>().ok())
                .as_deref(),
            Some("/tmp/fake.png")
        );
    }

    #[test]
    fn normalize_hints_discards_malformed_image_data() {
        let mut raw = HashMap::new();
        raw.insert("urgency", Value::new(1u8));
        raw.insert("image-data", Value::new("not-an-image"));
        let encoded = to_bytes(Context::new_dbus(LE, 0), &raw).expect("hints should encode");

        assert!(
            encoded.deserialize::<IncomingHints<'_>>().is_err(),
            "invalid image-data should be rejected at the D-Bus boundary"
        );
    }
}
