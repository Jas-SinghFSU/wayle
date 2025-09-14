mod notification;

pub use notification::*;

/// D-Bus interface constants for the notification service.
pub mod dbus {
    /// The D-Bus service name for notifications.
    pub const SERVICE: &str = "org.freedesktop.Notifications";

    /// The D-Bus object path for notifications.
    pub const PATH: &str = "/org/freedesktop/Notifications";

    /// The D-Bus interface name for notifications.
    pub const INTERFACE: &str = "org.freedesktop.Notifications";
}

/// D-Bus signal names for the notification service.
pub enum Signal {
    /// Signal emitted when a notification is closed.
    NotificationClosed,
    /// Signal emitted when an action is invoked on a notification.
    ActionInvoked,
    /// Signal emitted with an activation token.
    ActivationToken,
}

impl Signal {
    /// Converts the signal to its D-Bus string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Signal::NotificationClosed => "NotificationClosed",
            Signal::ActionInvoked => "ActionInvoked",
            Signal::ActivationToken => "ActivationToken",
        }
    }
}
