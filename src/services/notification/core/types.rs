use std::collections::HashMap;

use chrono::{DateTime, Utc};
use zbus::zvariant::OwnedValue;

#[derive(Debug, Clone)]
pub(crate) struct NotificationProps {
    pub id: u32,
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, OwnedValue>,
    pub expire_timeout: i32,
    pub timestamp: DateTime<Utc>,
}

/// Hints for notifications as specified by the Desktop Notifications Specification.
pub type NotificationHints = HashMap<String, OwnedValue>;

/// Represents a notification action with an ID and label.
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    /// The action identifier (e.g., "reply", "mark-read").
    pub id: String,
    /// The human-readable label (e.g., "Reply", "Mark as Read").
    pub label: String,
}

impl Action {
    /// Parses D-Bus action array into structured Action items.
    ///
    /// D-Bus sends actions as alternating id/label pairs:
    /// ["reply", "Reply", "delete", "Delete"] -> [Action{id: "reply", label: "Reply"}, ...]
    pub(crate) fn parse_dbus_actions(raw_actions: &[String]) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut iter = raw_actions.iter();

        while let Some(id) = iter.next() {
            let label = iter.next().unwrap_or(id);
            actions.push(Action {
                id: id.clone(),
                label: label.clone(),
            });
        }

        actions
    }

    pub(crate) fn to_dbus_format(actions: &[Action]) -> Vec<String> {
        let mut raw = Vec::with_capacity(actions.len() * 2);

        for action in actions {
            raw.push(action.id.clone());
            raw.push(action.label.clone());
        }

        raw
    }
}
