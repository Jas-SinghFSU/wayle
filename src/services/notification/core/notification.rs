use std::cmp::PartialEq;

use zbus::Connection;

use super::{
    controls::NotificationControls,
    types::{NotificationHints, NotificationProps},
};
use crate::services::{
    common::Property,
    notification::{
        error::Error,
        types::{Category, Urgency},
    },
};

/// A desktop notification.
///
/// Each notification displayed is allocated a unique ID by the server. This is unique
/// within the session. While the notification server is running, the ID will not be
/// recycled unless the capacity of a uint32 is exceeded.
#[derive(Clone, Debug)]
pub struct Notification {
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
    /// Actions are sent over as a list of pairs. Each even element in the list (starting at
    /// index 0) represents the identifier for the action. Each odd element in the list is
    /// the localized string that will be displayed to the user.
    ///
    /// The default action (usually invoked by clicking the notification) should have a key
    /// named "default". The name can be anything, though implementations are free not to
    /// display it.
    pub actions: Property<Vec<String>>,
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
    pub async fn dismiss(&self) -> Result<(), Error> {
        NotificationControls::dismiss(&self.zbus_connection, &self.id).await
    }

    /// Invoke an action on the notification.
    ///
    /// # Errors
    /// Returns error if the D-Bus signal emission fails.
    pub async fn invoke(&self, action_key: &str) -> Result<(), Error> {
        NotificationControls::invoke(&self.zbus_connection, &self.id, action_key).await
    }

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

        let id = if props.replaces_id > 0 {
            props.replaces_id
        } else {
            rand::random::<u32>().max(1)
        };

        Self {
            zbus_connection: connection.clone(),
            id,
            app_name: Property::new(app_name),
            app_icon: Property::new(app_icon),
            replaces_id: Property::new(replaces_id),
            summary: Property::new(props.summary),
            actions: Property::new(props.actions),
            body: Property::new(body),
            hints: Property::new(hints),
            expire_timeout: Property::new(expire_timeout),
            urgency: Property::new(*urgency),
            category: Property::new(category),
        }
    }
}
