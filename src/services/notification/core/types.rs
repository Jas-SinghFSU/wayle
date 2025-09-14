use std::collections::HashMap;

use zbus::{Connection, zvariant::Value};

#[derive(Debug, Clone)]
pub(crate) struct NotificationProps<'a> {
    pub connection: &'a Connection,
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, Value<'static>>,
    pub expire_timeout: i32,
}

/// Hints for notifications as specified by the Desktop Notifications Specification.
pub type NotificationHints = HashMap<String, Value<'static>>;
