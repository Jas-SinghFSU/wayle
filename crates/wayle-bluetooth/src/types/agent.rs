use std::fmt::{Display, Formatter, Result};

use tokio::sync::oneshot::Sender;
use zbus::zvariant::OwnedObjectPath;

/// Agent capability for pairing operations.
///
/// Describes the input/output capabilities of the agent for pairing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentCapability {
    /// Can display information and accept yes/no input
    DisplayYesNo,
    /// Can only display information, no input
    DisplayOnly,
    /// Can input text but cannot display
    KeyboardOnly,
    /// Can both display and input text
    #[default]
    KeyboardDisplay,
    /// No input or output capabilities
    NoInputNoOutput,
}

impl From<&str> for AgentCapability {
    fn from(s: &str) -> Self {
        match s {
            "DisplayYesNo" => Self::DisplayYesNo,
            "DisplayOnly" => Self::DisplayOnly,
            "KeyboardOnly" => Self::KeyboardOnly,
            "KeyboardDisplay" => Self::KeyboardDisplay,
            "NoInputNoOutput" => Self::NoInputNoOutput,
            "" => Self::KeyboardDisplay,
            _ => Self::KeyboardDisplay,
        }
    }
}

impl Display for AgentCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::DisplayYesNo => write!(f, "DisplayYesNo"),
            Self::DisplayOnly => write!(f, "DisplayOnly"),
            Self::KeyboardOnly => write!(f, "KeyboardOnly"),
            Self::KeyboardDisplay => write!(f, "KeyboardDisplay"),
            Self::NoInputNoOutput => write!(f, "NoInputNoOutput"),
        }
    }
}

/// Represents different pairing and authorization requests from BlueZ that require user interaction.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PairingRequest {
    /// Requests a PIN code from the user for legacy pairing.
    RequestPinCode {
        /// D-Bus object path of the device requesting PIN.
        device_path: OwnedObjectPath,
    },

    /// Displays a PIN code that the user must enter on the remote device.
    DisplayPinCode {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// 6-digit PIN to display (always zero-padded).
        pincode: String,
    },

    /// Requests a numeric passkey from the user.
    RequestPasskey {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
    },

    /// Displays a passkey that the user must enter on the remote device.
    DisplayPasskey {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// 6-digit passkey to display.
        passkey: u32,
        /// Number of digits already typed on remote side.
        entered: u16,
    },

    /// Requests confirmation that a passkey matches what's shown on the remote device.
    RequestConfirmation {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// 6-digit passkey to confirm.
        passkey: u32,
    },

    /// Requests authorization for pairing that would normally use just-works model.
    RequestAuthorization {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
    },

    /// Requests authorization for a specific service/profile connection.
    RequestServiceAuthorization {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// Service UUID requesting authorization.
        uuid: String,
    },
}

#[derive(Debug)]
pub(crate) enum PairingResponder {
    Pin(Sender<String>),
    Passkey(Sender<u32>),
    Confirmation(Sender<bool>),
    Authorization(Sender<bool>),
    ServiceAuthorization(Sender<bool>),
}

/// Agent events for Bluetooth pairing operations.
#[derive(Debug)]
pub(crate) enum AgentEvent {
    /// PIN code requested from user.
    PinRequested {
        /// D-Bus object path of the device requesting PIN.
        device_path: OwnedObjectPath,
        /// Channel to send PIN response.
        responder: Sender<String>,
    },
    /// Display PIN code to user.
    DisplayPinCode {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// PIN code to display.
        pincode: String,
    },
    /// Passkey requested from user.
    PasskeyRequested {
        /// D-Bus object path of the device requesting passkey.
        device_path: OwnedObjectPath,
        /// Channel to send passkey response.
        responder: Sender<u32>,
    },
    /// Display passkey to user.
    DisplayPasskey {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// Passkey to display.
        passkey: u32,
        /// Number of characters entered.
        entered: u16,
    },
    /// Confirmation requested from user.
    ConfirmationRequested {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// Passkey to confirm.
        passkey: u32,
        /// Channel to send confirmation response.
        responder: Sender<bool>,
    },
    /// Authorization requested from user.
    AuthorizationRequested {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// Channel to send authorization response.
        responder: Sender<bool>,
    },
    /// Service authorization requested from user.
    ServiceAuthorizationRequested {
        /// D-Bus object path of the device.
        device_path: OwnedObjectPath,
        /// Service UUID requiring authorization.
        uuid: String,
        /// Channel to send authorization response.
        responder: Sender<bool>,
    },
    /// Pairing request was cancelled.
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_capability_from_str_handles_all_variants() {
        assert_eq!(
            AgentCapability::from("DisplayYesNo"),
            AgentCapability::DisplayYesNo
        );
        assert_eq!(
            AgentCapability::from("DisplayOnly"),
            AgentCapability::DisplayOnly
        );
        assert_eq!(
            AgentCapability::from("KeyboardOnly"),
            AgentCapability::KeyboardOnly
        );
        assert_eq!(
            AgentCapability::from("KeyboardDisplay"),
            AgentCapability::KeyboardDisplay
        );
        assert_eq!(
            AgentCapability::from("NoInputNoOutput"),
            AgentCapability::NoInputNoOutput
        );
    }

    #[test]
    fn agent_capability_from_empty_string_returns_keyboard_display() {
        assert_eq!(AgentCapability::from(""), AgentCapability::KeyboardDisplay);
    }

    #[test]
    fn agent_capability_from_unknown_defaults_to_keyboard_display() {
        assert_eq!(
            AgentCapability::from("unknown"),
            AgentCapability::KeyboardDisplay
        );
        assert_eq!(
            AgentCapability::from("invalid"),
            AgentCapability::KeyboardDisplay
        );
    }
}
