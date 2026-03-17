use std::{
    collections::HashMap,
    env, fs,
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::{DateTime, Utc};
use derive_more::Debug;
use rusqlite::{Connection, params};
use tracing::{debug, instrument, warn};
use zbus::zvariant::{OwnedValue, Str};

use crate::{
    core::{
        notification::Notification,
        types::{Action, IMAGE_DATA_KEYS},
    },
    error::Error,
};

#[derive(Debug)]
pub(crate) struct StoredNotification {
    pub id: u32,
    pub app_name: Option<String>,
    pub replaces_id: Option<u32>,
    pub app_icon: Option<String>,
    pub summary: String,
    pub body: Option<String>,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub image_path: Option<String>,
    pub expire_timeout: Option<u32>,
    pub timestamp: i64,
}

impl From<&Notification> for StoredNotification {
    fn from(notification: &Notification) -> Self {
        Self {
            id: notification.id,
            app_name: notification.app_name.get().clone(),
            replaces_id: notification.replaces_id.get(),
            app_icon: notification.app_icon.get().clone(),
            summary: notification.summary.get().clone(),
            body: notification.body.get().clone(),
            actions: Action::to_dbus_format(&notification.actions.get()),
            hints: notification.hints.get().clone().unwrap_or_default(),
            image_path: notification.image_path.get().clone(),
            expire_timeout: notification.expire_timeout.get(),
            timestamp: notification.timestamp.get().timestamp_millis(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationStore {
    #[debug(skip)]
    connection: Arc<Mutex<Connection>>,
}

impl NotificationStore {
    #[instrument(err)]
    pub fn new() -> Result<Self, Error> {
        let home = env::var("HOME")
            .map_err(|_| Error::DatabaseError(String::from("HOME environment variable not set")))?;

        let data_dir = format!("{home}/.local/share/wayle");
        fs::create_dir_all(&data_dir)
            .map_err(|err| Error::DatabaseError(format!("cannot create data directory: {err}")))?;

        let db_path = format!("{data_dir}/notifications.db");
        debug!(path = %db_path, "notification store opened");
        let connection = Connection::open(db_path)
            .map_err(|err| Error::DatabaseError(format!("cannot open database: {err}")))?;

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS notifications (
                    id INTEGER PRIMARY KEY,
                    app_name TEXT,
                    replaces_id INTEGER,
                    app_icon TEXT,
                    summary TEXT NOT NULL,
                    body TEXT,
                    actions TEXT NOT NULL,
                    hints TEXT NOT NULL,
                    expire_timeout INTEGER,
                    timestamp INTEGER NOT NULL,
                    image_path TEXT
                )",
                [],
            )
            .map_err(|err| Error::DatabaseError(format!("cannot create table: {err}")))?;

        connection
            .execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;",
            )
            .map_err(|err| Error::DatabaseError(format!("cannot set pragmas: {err}")))?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    #[instrument(skip(self, notification), fields(id = notification.id, summary = %notification.summary.get()), err)]
    pub fn add(&self, notification: &Notification) -> Result<(), Error> {
        let stored = StoredNotification::from(notification);

        let actions_json = serde_json::to_string(&stored.actions)
            .map_err(|err| Error::DatabaseError(format!("cannot serialize actions: {err}")))?;

        let mut hints_for_storage = stored.hints.clone();
        for key in &IMAGE_DATA_KEYS {
            hints_for_storage.remove(*key);
        }
        let hints_json = serde_json::to_string(&hints_for_storage)
            .map_err(|err| Error::DatabaseError(format!("cannot serialize hints: {err}")))?;

        self.connection
            .lock()
            .map_err(|_| Error::DatabaseError("cannot acquire lock on database".to_string()))?
            .execute(
                "INSERT OR REPLACE INTO notifications
                 (id, app_name, replaces_id, app_icon, summary, body, actions, hints,
                 expire_timeout, timestamp, image_path)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    stored.id,
                    stored.app_name,
                    stored.replaces_id,
                    stored.app_icon,
                    stored.summary,
                    stored.body,
                    actions_json,
                    hints_json,
                    stored.expire_timeout,
                    stored.timestamp,
                    stored.image_path,
                ],
            )
            .map_err(|err| Error::DatabaseError(format!("cannot store notification: {err}")))?;

        Ok(())
    }

    #[instrument(skip(self), fields(notification_id = id), err)]
    pub fn remove(&self, id: u32) -> Result<(), Error> {
        self.connection
            .lock()
            .map_err(|_| Error::DatabaseError("cannot acquire lock on database".to_string()))?
            .execute("DELETE FROM notifications WHERE id = ?1", params![id])
            .map_err(|err| Error::DatabaseError(format!("cannot remove notification: {err}")))?;

        Ok(())
    }

    #[instrument(skip(self), err)]
    pub fn load_all(&self, remove_expired: bool) -> Result<Vec<StoredNotification>, Error> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| Error::DatabaseError("cannot acquire lock on database".to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, app_name, replaces_id, app_icon, summary, body,
                 actions, hints, expire_timeout, timestamp, image_path
                 FROM notifications
                 ORDER BY timestamp DESC",
            )
            .map_err(|err| Error::DatabaseError(format!("cannot prepare query: {err}")))?;

        let notifications = stmt
            .query_map([], |row| {
                let actions_json: String = row.get(6)?;
                let hints_json: String = row.get(7)?;
                let image_path: Option<String> = row.get(10)?;

                let actions: Vec<String> =
                    serde_json::from_str(&actions_json).unwrap_or_else(|err| {
                        warn!(error = %err, "cannot deserialize actions");
                        Vec::new()
                    });
                let hints_json_map: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&hints_json).unwrap_or_else(|err| {
                        warn!(error = %err, "cannot deserialize hints");
                        HashMap::new()
                    });
                let mut hints: HashMap<String, OwnedValue> = hints_json_map
                    .into_iter()
                    .filter_map(|(key, value)| {
                        serde_json::from_value::<OwnedValue>(value)
                            .ok()
                            .map(|owned_value| (key, owned_value))
                    })
                    .collect();

                if let Some(ref path) = image_path {
                    hints.insert(
                        String::from("image-path"),
                        OwnedValue::from(Str::from(path.as_str())),
                    );
                }

                Ok(StoredNotification {
                    id: row.get(0)?,
                    app_name: row.get(1)?,
                    replaces_id: row.get(2)?,
                    app_icon: row.get(3)?,
                    summary: row.get(4)?,
                    body: row.get(5)?,
                    actions,
                    hints,
                    image_path,
                    expire_timeout: row.get(8)?,
                    timestamp: row.get(9)?,
                })
            })
            .map_err(|err| Error::DatabaseError(format!("cannot query notifications: {err}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| Error::DatabaseError(format!("cannot parse notifications: {err}")))?;

        if !remove_expired {
            debug!(count = notifications.len(), "loaded stored notifications");
            return Ok(notifications);
        }

        let now = Utc::now();
        let notifications: Vec<StoredNotification> = notifications
            .into_iter()
            .filter(|notif| {
                let Some(timeout) = notif.expire_timeout else {
                    return true;
                };
                let Some(timestamp) = DateTime::<Utc>::from_timestamp_millis(notif.timestamp)
                else {
                    return false;
                };
                timestamp + Duration::from_millis(timeout as u64) > now
            })
            .collect();

        debug!(
            count = notifications.len(),
            "loaded stored notifications (expired filtered)"
        );
        Ok(notifications)
    }
}
