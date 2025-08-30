use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;
use zbus::zvariant::OwnedObjectPath;

/// Agent capability for pairing operations.
///
/// Describes the input/output capabilities of the agent for pairing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentCapability {
    /// Can display information and accept yes/no input
    DisplayYesNo,
    /// Can only display information, no input
    DisplayOnly,
    /// Can input text but cannot display
    KeyboardOnly,
    /// Can both display and input text
    KeyboardDisplay,
    /// No input or output capabilities
    NoInputNoOutput,
}

impl Default for AgentCapability {
    fn default() -> Self {
        Self::KeyboardDisplay
    }
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
    RequestPinCode,

    /// Displays a PIN code that the user must enter on the remote device.
    DisplayPinCode {
        /// 6-digit PIN to display (always zero-padded).
        pincode: String,
    },

    /// Requests a numeric passkey from the user.
    RequestPasskey,

    /// Displays a passkey that the user must enter on the remote device.
    DisplayPasskey {
        /// 6-digit passkey to display.
        passkey: u32,
        /// Number of digits already typed on remote side.
        entered: u16,
    },

    /// Requests confirmation that a passkey matches what's shown on the remote device.
    RequestConfirmation {
        /// 6-digit passkey to confirm.
        passkey: u32,
    },

    /// Requests authorization for pairing that would normally use just-works model.
    RequestAuthorization,

    /// Requests authorization for a specific service/profile connection.
    AuthorizeService {
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
}

// Event broadcast from Agent to ALL devices
#[derive(Debug)]
pub enum AgentEvent {
    PinRequested {
        device_path: OwnedObjectPath,
        responder: Sender<String>,
    },
    DisplayPinCode {
        device_path: OwnedObjectPath,
        pincode: String,
    },
    PasskeyRequested {
        device_path: OwnedObjectPath,
        responder: Sender<u32>,
    },
    DisplayPasskey {
        device_path: OwnedObjectPath,
        passkey: u32,
        entered: u16,
    },
    ConfirmationRequested {
        device_path: OwnedObjectPath,
        passkey: u32,
        responder: Sender<bool>,
    },
    AuthorizationRequested {
        device_path: OwnedObjectPath,
        responder: Sender<bool>,
    },
    ServiceAuthorizationRequested {
        device_path: OwnedObjectPath,
        uuid: String,
        responder: Sender<bool>,
    },
    Cancelled {
        device_path: OwnedObjectPath,
    },
}

impl AgentEvent {
    pub fn device_path(&self) -> &OwnedObjectPath {
        match self {
            Self::PinRequested { device_path, .. } => device_path,
            Self::DisplayPinCode { device_path, .. } => device_path,
            Self::PasskeyRequested { device_path, .. } => device_path,
            Self::DisplayPasskey { device_path, .. } => device_path,
            Self::ConfirmationRequested { device_path, .. } => device_path,
            Self::AuthorizationRequested { device_path, .. } => device_path,
            Self::ServiceAuthorizationRequested { device_path, .. } => device_path,
            Self::Cancelled { device_path } => device_path,
        }
    }
}
