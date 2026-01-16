mod notification;

pub use notification::*;

pub(crate) mod dbus {
    pub const SERVICE_NAME: &str = "org.freedesktop.Notifications";
    pub const SERVICE_PATH: &str = "/org/freedesktop/Notifications";
    pub const SERVICE_INTERFACE: &str = "org.freedesktop.Notifications";
    pub const WAYLE_SERVICE_NAME: &str = "com.wayle.Notifications1";
    pub const WAYLE_SERVICE_PATH: &str = "/com/wayle/Notifications";
}

pub(crate) type Name = String;
pub(crate) type Vendor = String;
pub(crate) type Version = String;
pub(crate) type SpecVersion = String;

pub(crate) enum Signal {
    NotificationClosed,
    ActionInvoked,
    ActivationToken,
}

impl Signal {
    pub(crate) fn as_str(&self) -> &'static str {
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
