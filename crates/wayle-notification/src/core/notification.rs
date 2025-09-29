use std::cmp::PartialEq;

use chrono::{DateTime, Utc};
use derive_more::Debug;
use tracing::instrument;
use wayle_common::Property;
use zbus::Connection;

use super::{
    controls::NotificationControls,
    types::{Action, NotificationHints, NotificationProps},
};
use crate::{
    error::Error,
    types::{Category, Urgency},
};

/// A desktop notification.
///
/// Each notification displayed is allocated a unique ID by the server. This is unique
/// within the session. While the notification server is running, the ID will not be
/// recycled unless the capacity of a uint32 is exceeded.
#[derive(Clone, Debug)]
pub struct Notification {
    #[debug(skip)]
    zbus_connection: Connection,

    /// The ID of the notification
    pub id: u32,
    /// The optional name of the application sending the notification. This should be the
    /// application's formal name, rather than some sort of ID. An example would be
    /// "FredApp E-Mail Client," rather than "fredapp-email-client."
    pub app_name: Property<Option<String>>,
    /// An optional ID of an existing notification that this notification is intended to replace.
    pub replaces_id: Property<Option<u32>>,
    /// The notification icon.
    pub app_icon: Property<Option<String>>,
    /// This is a single line overview of the notification. For instance, "You have mail"
    /// or "A friend has come online". It should generally not be longer than 40 characters,
    /// though this is not a requirement, and server implementations should word wrap if
    /// necessary. The summary must be encoded using UTF-8.
    pub summary: Property<String>,
    /// This is a multi-line body of text. Each line is a paragraph, server implementations
    /// are free to word wrap them as they see fit.
    ///
    /// The body may contain simple markup as specified in Markup. It must be encoded using UTF-8.
    ///
    /// If the body is omitted, just the summary is displayed.
    pub body: Property<Option<String>>,
    /// Available actions for this notification.
    ///
    /// Each action has an identifier and a human-readable label.
    /// The "default" action is typically invoked when clicking the notification body.
    pub actions: Property<Vec<Action>>,
    /// The default action, triggered when clicking the notification body.
    pub default_action: Property<Option<Action>>,
    /// Hints are a way to provide extra data to a notification server that the server may
    /// be able to make use of.
    ///
    /// Neither clients nor notification servers are required to support any hints. Both
    /// sides should assume that hints are not passed, and should ignore any hints they
    /// do not understand.
    pub hints: Property<Option<NotificationHints>>,
    /// The timeout time in milliseconds since the display of the notification at which
    /// the notification should automatically close.
    ///
    /// If None, the notification never expires.
    pub expire_timeout: Property<Option<u32>>,
    /// The urgency level.
    pub urgency: Property<Urgency>,
    /// The type of notification this is.
    pub category: Property<Option<Category>>,
    /// When the notification was created.
    pub timestamp: Property<DateTime<Utc>>,
    /// Path to an image file from hints.
    pub image_path: Property<Option<String>>,
    /// Desktop entry name of the application.
    pub desktop_entry: Property<Option<String>>,
    /// Whether the notification should be transient (not persisted).
    pub is_transient: Property<bool>,
    /// Whether the notification stays after action invocation.
    pub is_resident: Property<bool>,
    /// Path to a sound file to play when the notification pops up.
    pub sound_file: Property<Option<String>>,
    /// A themeable named sound to play when the notification pops up.
    pub sound_name: Property<Option<String>>,
    /// Whether to suppress playing sounds for this notification.
    pub suppress_sound: Property<bool>,
    /// X position hint for notification placement.
    pub x: Property<Option<i32>>,
    /// Y position hint for notification placement.
    pub y: Property<Option<i32>>,
    /// Whether action IDs should be interpreted as icon names.
    pub action_icons: Property<bool>,
}

impl PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Notification {
    pub(crate) fn new(props: NotificationProps, connection: Connection) -> Self {
        Self::from_props(props, connection)
    }

    /// Causes a notification to be forcefully closed and removed from the user's view.
    /// It can be used, for example, in the event that what the notification pertains to
    /// is no longer relevant, or to cancel a notification with no expiration time.
    ///
    /// The NotificationClosed signal is emitted by this method.
    ///
    /// # Errors
    /// Returns error if the D-Bus signal emission fails.
    #[instrument(skip(self), fields(notification_id = %self.id), err)]
    pub async fn dismiss(&self) -> Result<(), Error> {
        NotificationControls::dismiss(&self.zbus_connection, &self.id).await
    }

    /// Invoke an action on the notification.
    ///
    /// # Errors
    /// Returns error if the D-Bus signal emission fails.
    #[instrument(skip(self), fields(notification_id = %self.id, action = %action_key), err)]
    pub async fn invoke(&self, action_key: &str) -> Result<(), Error> {
        NotificationControls::invoke(&self.zbus_connection, &self.id, action_key).await
    }

    #[allow(clippy::too_many_lines)]
    fn from_props(props: NotificationProps, connection: Connection) -> Notification {
        let app_name = if !props.app_name.is_empty() {
            Some(props.app_name)
        } else {
            None
        };

        let app_icon = if !props.app_icon.is_empty() {
            Some(props.app_icon)
        } else {
            None
        };

        let replaces_id = if props.replaces_id > 0 {
            Some(props.replaces_id)
        } else {
            None
        };

        let body = if !props.body.is_empty() {
            Some(props.body)
        } else {
            None
        };

        let urgency = &props
            .hints
            .get("urgency")
            .and_then(|u| u.downcast_ref::<u8>().ok())
            .map_or(Urgency::Normal, Urgency::from);

        let category = props
            .hints
            .get("category")
            .and_then(|v| v.downcast_ref::<String>().ok())
            .and_then(|s| s.parse().ok());

        let image_path = props
            .hints
            .get("image-path")
            .and_then(|v| v.downcast_ref::<String>().ok());

        let desktop_entry = props
            .hints
            .get("desktop-entry")
            .and_then(|v| v.downcast_ref::<String>().ok());

        let is_transient = props
            .hints
            .get("transient")
            .and_then(|v| v.downcast_ref::<bool>().ok())
            .unwrap_or(false);

        let is_resident = props
            .hints
            .get("resident")
            .and_then(|v| v.downcast_ref::<bool>().ok())
            .unwrap_or(false);

        let sound_file = props
            .hints
            .get("sound-file")
            .and_then(|v| v.downcast_ref::<String>().ok());

        let sound_name = props
            .hints
            .get("sound-name")
            .and_then(|v| v.downcast_ref::<String>().ok());

        let suppress_sound = props
            .hints
            .get("suppress-sound")
            .and_then(|v| v.downcast_ref::<bool>().ok())
            .unwrap_or(false);

        let x = props
            .hints
            .get("x")
            .and_then(|v| v.downcast_ref::<i32>().ok());

        let y = props
            .hints
            .get("y")
            .and_then(|v| v.downcast_ref::<i32>().ok());

        let action_icons = props
            .hints
            .get("action-icons")
            .and_then(|v| v.downcast_ref::<bool>().ok())
            .unwrap_or(false);

        let parsed_actions = Action::parse_dbus_actions(&props.actions);
        let default_action = parsed_actions.iter().find(|a| a.id == "default").cloned();

        let hints = if !props.hints.is_empty() {
            Some(props.hints)
        } else {
            None
        };

        let expire_timeout = if props.expire_timeout > 0 {
            Some(props.expire_timeout as u32)
        } else {
            None
        };

        let id = props.id;

        Self {
            zbus_connection: connection.clone(),
            id,
            app_name: Property::new(app_name),
            app_icon: Property::new(app_icon),
            replaces_id: Property::new(replaces_id),
            summary: Property::new(props.summary),
            actions: Property::new(parsed_actions),
            default_action: Property::new(default_action),
            body: Property::new(body),
            hints: Property::new(hints),
            expire_timeout: Property::new(expire_timeout),
            urgency: Property::new(*urgency),
            category: Property::new(category),
            timestamp: Property::new(props.timestamp),
            image_path: Property::new(image_path),
            desktop_entry: Property::new(desktop_entry),
            is_transient: Property::new(is_transient),
            is_resident: Property::new(is_resident),
            sound_file: Property::new(sound_file),
            sound_name: Property::new(sound_name),
            suppress_sound: Property::new(suppress_sound),
            x: Property::new(x),
            y: Property::new(y),
            action_icons: Property::new(action_icons),
        }
    }
}
