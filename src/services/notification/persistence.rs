use std::{
    collections::HashMap,
    env, fs,
    sync::{Arc, Mutex},
};

use rusqlite::{Connection, params};
use serde_json;
use zbus::zvariant::OwnedValue;

use super::{
    core::{notification::Notification, types::Action},
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
            expire_timeout: notification.expire_timeout.get(),
            timestamp: notification.timestamp.get().timestamp_millis(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationStore {
    connection: Arc<Mutex<Connection>>,
}

impl NotificationStore {
    pub fn new() -> Result<Self, Error> {
        let home = env::var("HOME")
            .map_err(|_| Error::DatabaseError(String::from("HOME environment variable not set")))?;

        let data_dir = format!("{home}/.local/share/wayle");
        fs::create_dir_all(&data_dir)
            .map_err(|e| Error::DatabaseError(format!("Failed to create data directory: {e}")))?;

        let db_path = format!("{data_dir}/notifications.db");
        let connection = Connection::open(db_path)
            .map_err(|e| Error::DatabaseError(format!("Failed to open database: {e}")))?;

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
                    urgency INTEGER NOT NULL,
                    category TEXT,
                    timestamp INTEGER NOT NULL
                )",
                [],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to create table: {e}")))?;

        connection
            .execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;",
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to set pragmas: {e}")))?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn add(&self, notification: &Notification) -> Result<(), Error> {
        let stored = StoredNotification::from(notification);

        let actions_json = serde_json::to_string(&stored.actions)
            .map_err(|e| Error::DatabaseError(format!("Failed to serialize actions: {e}")))?;
        let hints_json = serde_json::to_string(&stored.hints)
            .map_err(|e| Error::DatabaseError(format!("Failed to serialize hints: {e}")))?;

        self.connection
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock on database".to_string()))?
            .execute(
                "INSERT OR REPLACE INTO notifications
                 (id, app_name, replaces_id, app_icon, summary, body, actions, hints,
                 expire_timeout, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
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
                ],
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to store notification: {e}")))?;

        Ok(())
    }

    pub fn remove(&self, id: u32) -> Result<(), Error> {
        self.connection
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock on database".to_string()))?
            .execute("DELETE FROM notifications WHERE id = ?1", params![id])
            .map_err(|e| Error::DatabaseError(format!("Failed to remove notification: {e}")))?;

        Ok(())
    }

    pub fn load_all(&self) -> Result<Vec<StoredNotification>, Error> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| Error::DatabaseError("Failed to acquire lock on database".to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, app_name, replaces_id, app_icon, summary, body,
                 actions, hints, expire_timeout
                 FROM notifications
                 ORDER BY timestamp DESC",
            )
            .map_err(|e| Error::DatabaseError(format!("Failed to prepare query: {e}")))?;

        let notifications = stmt
            .query_map([], |row| {
                let actions_json: String = row.get(6)?;
                let hints_json: String = row.get(7)?;

                let actions: Vec<String> = serde_json::from_str(&actions_json).unwrap_or_default();
                let hints_json_map: HashMap<String, serde_json::Value> =
                    serde_json::from_str(&hints_json).unwrap_or_default();
                let hints: HashMap<String, OwnedValue> = hints_json_map
                    .into_iter()
                    .filter_map(|(key, value)| {
                        serde_json::from_value::<OwnedValue>(value)
                            .ok()
                            .map(|owned_value| (key, owned_value))
                    })
                    .collect();

                Ok(StoredNotification {
                    id: row.get(0)?,
                    app_name: row.get(1)?,
                    replaces_id: row.get(2)?,
                    app_icon: row.get(3)?,
                    summary: row.get(4)?,
                    body: row.get(5)?,
                    actions,
                    hints,
                    expire_timeout: row.get(8)?,
                    timestamp: row.get(9)?,
                })
            })
            .map_err(|e| Error::DatabaseError(format!("Failed to query notifications: {e}")))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::DatabaseError(format!("Failed to parse notifications: {e}")))?;

        Ok(notifications)
    }
}
