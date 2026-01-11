mod notification;

pub use notification::*;

/// D-Bus interface constants for the notification service.
pub mod dbus {
    /// The D-Bus service name for notifications (freedesktop).
    pub const SERVICE_NAME: &str = "org.freedesktop.Notifications";

    /// The D-Bus object path for notifications (freedesktop).
    pub const SERVICE_PATH: &str = "/org/freedesktop/Notifications";

    /// The D-Bus interface name for notifications (freedesktop).
    pub const SERVICE_INTERFACE: &str = "org.freedesktop.Notifications";

    /// Wayle extensions D-Bus service name.
    pub const WAYLE_SERVICE_NAME: &str = "com.wayle.Notifications1";

    /// Wayle extensions D-Bus object path.
    pub const WAYLE_SERVICE_PATH: &str = "/com/wayle/Notifications";
}

pub(crate) type Name = String;
pub(crate) type Vendor = String;
pub(crate) type Version = String;
pub(crate) type SpecVersion = String;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_notification_closed_returns_correct_str() {
        let signal = Signal::NotificationClosed;

        let result = signal.as_str();

        assert_eq!(result, "NotificationClosed");
    }

    #[test]
    fn signal_action_invoked_returns_correct_str() {
        let signal = Signal::ActionInvoked;

        let result = signal.as_str();

        assert_eq!(result, "ActionInvoked");
    }

    #[test]
    fn signal_activation_token_returns_correct_str() {
        let signal = Signal::ActivationToken;

        let result = signal.as_str();

        assert_eq!(result, "ActivationToken");
    }
}
