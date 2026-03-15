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

/// Raw pixel data from the `image-data` notification hint.
///
/// Matches the D-Bus `(iiibiiay)` structure from the
/// [freedesktop icons and images spec](https://specifications.freedesktop.org/notification/1.2/icons-and-images.html).
/// `bits_per_sample` is always 8; `channels` is 3 (RGB) or 4 (RGBA).
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Image width in pixels.
    pub width: i32,
    /// Image height in pixels.
    pub height: i32,
    /// Distance in bytes between row starts (may include padding).
    pub rowstride: i32,
    /// Whether the image has an alpha channel.
    pub has_alpha: bool,
    /// Bits per sample (always 8 per spec).
    pub bits_per_sample: i32,
    /// Number of channels (3 for RGB, 4 for RGBA).
    pub channels: i32,
    /// Raw pixel data in RGB or RGBA byte order.
    pub data: Vec<u8>,
}

pub(crate) const IMAGE_DATA_KEYS: [&str; 3] = ["image-data", "image_data", "icon_data"];

impl ImageData {
    /// Extracts `image-data` from notification hints.
    ///
    /// Tries the current key (`image-data`) and deprecated fallbacks
    /// (`image_data`, `icon_data`) in priority order per the spec.
    pub(crate) fn from_hints(hints: &NotificationHints) -> Option<Self> {
        type Tuple = (i32, i32, i32, bool, i32, i32, Vec<u8>);

        for key in &IMAGE_DATA_KEYS {
            let Some(value) = hints.get(*key) else {
                continue;
            };

            let Ok((width, height, rowstride, has_alpha, bits_per_sample, channels, data)) =
                Tuple::try_from(value.clone())
            else {
                continue;
            };

            return Some(Self {
                width,
                height,
                rowstride,
                has_alpha,
                bits_per_sample,
                channels,
                data,
            });
        }

        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dbus_actions_with_empty_input_returns_empty_vec() {
        let raw_actions: Vec<String> = vec![];

        let result = Action::parse_dbus_actions(&raw_actions);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn parse_dbus_actions_with_even_count_creates_actions() {
        let raw_actions = vec![
            "reply".to_string(),
            "Reply".to_string(),
            "delete".to_string(),
            "Delete".to_string(),
        ];

        let result = Action::parse_dbus_actions(&raw_actions);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, "reply");
        assert_eq!(result[0].label, "Reply");
        assert_eq!(result[1].id, "delete");
        assert_eq!(result[1].label, "Delete");
    }

    #[test]
    fn parse_dbus_actions_with_odd_count_uses_id_as_label_for_last() {
        let raw_actions = vec![
            "reply".to_string(),
            "Reply".to_string(),
            "default".to_string(),
        ];

        let result = Action::parse_dbus_actions(&raw_actions);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, "reply");
        assert_eq!(result[0].label, "Reply");
        assert_eq!(result[1].id, "default");
        assert_eq!(result[1].label, "default");
    }

    #[test]
    fn to_dbus_format_with_empty_input_returns_empty_vec() {
        let actions: Vec<Action> = vec![];

        let result = Action::to_dbus_format(&actions);

        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn to_dbus_format_creates_alternating_id_label_pairs() {
        let actions = vec![
            Action {
                id: "reply".to_string(),
                label: "Reply".to_string(),
            },
            Action {
                id: "delete".to_string(),
                label: "Delete".to_string(),
            },
        ];

        let result = Action::to_dbus_format(&actions);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "reply");
        assert_eq!(result[1], "Reply");
        assert_eq!(result[2], "delete");
        assert_eq!(result[3], "Delete");
    }

    #[test]
    fn parse_and_to_dbus_format_are_inverse_operations() {
        let original = vec![
            "reply".to_string(),
            "Reply".to_string(),
            "mark-read".to_string(),
            "Mark as Read".to_string(),
        ];

        let parsed = Action::parse_dbus_actions(&original);
        let result = Action::to_dbus_format(&parsed);

        assert_eq!(result, original);
    }
}
