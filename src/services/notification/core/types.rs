use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

#[derive(Debug, Clone)]
pub(crate) struct NotificationProps {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub expire_timeout: i32,
}

/// Hints for notifications as specified by the Desktop Notifications Specification.
pub type NotificationHints = HashMap<String, OwnedValue>;
