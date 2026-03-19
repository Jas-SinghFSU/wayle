use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use zbus::zvariant::{OwnedValue, Type, as_value::optional};

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

#[derive(Debug, Clone, Copy)]
pub(crate) struct BorrowedImageData<'a> {
    /// Image width in pixels.
    pub width: i32,
    /// Image height in pixels.
    pub height: i32,
    /// Distance in bytes between row starts (may include padding).
    pub rowstride: i32,
    /// Bits per sample (always 8 per spec).
    pub bits_per_sample: i32,
    /// Number of channels (3 for RGB, 4 for RGBA).
    pub channels: i32,
    /// Borrowed raw pixel data in RGB or RGBA byte order.
    pub data: &'a [u8],
}

pub(crate) const IMAGE_DATA_KEYS: [&str; 3] = ["image-data", "image_data", "icon_data"];

/// Hints for notifications as specified by the Desktop Notifications Specification.
pub type NotificationHints = HashMap<String, OwnedValue>;

type RawImageData<'a> = (i32, i32, i32, bool, i32, i32, &'a [u8]);

#[derive(Debug, Default, Deserialize, Type)]
#[serde(default)]
#[zvariant(signature = "a{sv}")]
pub(crate) struct IncomingHints<'a> {
    #[serde(borrow, with = "optional", rename = "image-data")]
    image_data: Option<RawImageData<'a>>,
    #[serde(borrow, with = "optional", rename = "image_data")]
    image_data_legacy: Option<RawImageData<'a>>,
    #[serde(borrow, with = "optional", rename = "icon_data")]
    icon_data: Option<RawImageData<'a>>,
    #[serde(flatten)]
    hints: NotificationHints,
}

impl<'a> IncomingHints<'a> {
    pub(crate) fn image_data(&self) -> Option<BorrowedImageData<'a>> {
        let (width, height, rowstride, _has_alpha, bits_per_sample, channels, data) = self
            .image_data
            .or(self.image_data_legacy)
            .or(self.icon_data)?;

        Some(BorrowedImageData {
            width,
            height,
            rowstride,
            bits_per_sample,
            channels,
            data,
        })
    }

    pub(crate) fn into_owned(self) -> NotificationHints {
        self.hints
    }
}

/// Represents a notification action with an ID and label.
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    /// Action identifier sent via D-Bus `ActionInvoked` signal.
    pub id: String,
    /// Human-readable label for display.
    pub label: String,
}

impl Action {
    /// The spec-defined identifier for the body-click action.
    pub const DEFAULT_ID: &str = "default";
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
    use zbus::zvariant::{LE, Value, serialized::Context, to_bytes};

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

    #[test]
    fn incoming_hints_extracts_image_data_without_storing_raw_hint() {
        let pixels = [0u8, 1, 2, 3];
        let mut raw = HashMap::new();
        raw.insert("category", Value::new("im.received"));
        raw.insert(
            "image-data",
            Value::new((1i32, 1i32, 4i32, true, 8i32, 4i32, &pixels[..])),
        );
        let encoded = to_bytes(Context::new_dbus(LE, 0), &raw).expect("hints should encode");

        let (hints, _): (IncomingHints<'_>, _) = encoded.deserialize().expect("hints should parse");

        let image = hints.image_data().expect("image-data should parse");

        assert_eq!(image.width, 1);
        assert_eq!(image.height, 1);
        assert_eq!(image.data, pixels);
        assert!(hints.into_owned().contains_key("category"));
    }

    #[test]
    fn incoming_hints_prefers_spec_image_data_key() {
        let low_priority = [9u8, 9, 9, 9];
        let high_priority = [1u8, 2, 3, 4];
        let mut raw = HashMap::new();
        raw.insert(
            "icon_data",
            Value::new((1i32, 1i32, 4i32, true, 8i32, 4i32, &low_priority[..])),
        );
        raw.insert(
            "image-data",
            Value::new((1i32, 1i32, 4i32, true, 8i32, 4i32, &high_priority[..])),
        );
        let encoded = to_bytes(Context::new_dbus(LE, 0), &raw).expect("hints should encode");

        let (hints, _): (IncomingHints<'_>, _) = encoded.deserialize().expect("hints should parse");

        assert_eq!(
            hints.image_data().expect("image-data should parse").data,
            high_priority
        );
    }
}
