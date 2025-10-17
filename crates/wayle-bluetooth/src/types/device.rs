use std::{
    self,
    fmt::{Display, Formatter, Result},
};

use serde::{Deserialize, Serialize};

/// Preferred bearer for dual-mode Bluetooth devices.
///
/// [experimental]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferredBearer {
    /// Connect to last used bearer first (default)
    #[serde(rename = "last-used")]
    LastUsed,
    /// Connect to BR/EDR first
    #[serde(rename = "bredr")]
    BrEdr,
    /// Connect to LE first
    #[serde(rename = "le")]
    Le,
    /// Connect to last seen bearer first
    #[serde(rename = "last-seen")]
    LastSeen,
}

impl Default for PreferredBearer {
    fn default() -> Self {
        Self::LastUsed
    }
}

impl From<&str> for PreferredBearer {
    fn from(s: &str) -> Self {
        match s {
            "last-used" => Self::LastUsed,
            "bredr" => Self::BrEdr,
            "le" => Self::Le,
            "last-seen" => Self::LastSeen,
            _ => Self::LastUsed,
        }
    }
}

impl Display for PreferredBearer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::LastUsed => write!(f, "last-used"),
            Self::BrEdr => write!(f, "bredr"),
            Self::Le => write!(f, "le"),
            Self::LastSeen => write!(f, "last-seen"),
        }
    }
}

/// Bluetooth device disconnection reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisconnectReason {
    /// Unknown reason
    Unknown,
    /// Connection timeout
    ConnectionTimeout,
    /// Connection terminated by local host
    ConnectionTerminatedLocal,
    /// Connection terminated by remote host
    ConnectionTerminatedRemote,
    /// Authentication failure
    AuthenticationFailure,
    /// Connection terminated due to suspend
    Suspend,
}

impl From<&str> for DisconnectReason {
    fn from(s: &str) -> Self {
        match s {
            "org.bluez.Reason.Timeout" => Self::ConnectionTimeout,
            "org.bluez.Reason.Local" => Self::ConnectionTerminatedLocal,
            "org.bluez.Reason.Remote" => Self::ConnectionTerminatedRemote,
            "org.bluez.Reason.Authentication" => Self::AuthenticationFailure,
            "org.bluez.Reason.Suspend" => Self::Suspend,
            _ => Self::Unknown,
        }
    }
}

impl From<u8> for DisconnectReason {
    fn from(code: u8) -> Self {
        match code {
            0x08 => Self::ConnectionTimeout,
            0x16 => Self::ConnectionTerminatedLocal,
            0x13 => Self::ConnectionTerminatedRemote,
            0x05 => Self::AuthenticationFailure,
            _ => Self::Unknown,
        }
    }
}

impl Display for DisconnectReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::ConnectionTimeout => write!(f, "Connection timeout"),
            Self::ConnectionTerminatedLocal => write!(f, "Connection terminated by local host"),
            Self::ConnectionTerminatedRemote => write!(f, "Connection terminated by remote host"),
            Self::AuthenticationFailure => write!(f, "Authentication failure"),
            Self::Suspend => write!(f, "Suspend"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preferred_bearer_from_str_handles_all_variants() {
        assert_eq!(
            PreferredBearer::from("last-used"),
            PreferredBearer::LastUsed
        );
        assert_eq!(PreferredBearer::from("bredr"), PreferredBearer::BrEdr);
        assert_eq!(PreferredBearer::from("le"), PreferredBearer::Le);
        assert_eq!(
            PreferredBearer::from("last-seen"),
            PreferredBearer::LastSeen
        );
    }

    #[test]
    fn preferred_bearer_from_str_defaults_to_last_used() {
        assert_eq!(PreferredBearer::from("unknown"), PreferredBearer::LastUsed);
        assert_eq!(PreferredBearer::from(""), PreferredBearer::LastUsed);
    }

    #[test]
    fn disconnect_reason_from_str_handles_all_variants() {
        assert_eq!(
            DisconnectReason::from("org.bluez.Reason.Timeout"),
            DisconnectReason::ConnectionTimeout
        );
        assert_eq!(
            DisconnectReason::from("org.bluez.Reason.Local"),
            DisconnectReason::ConnectionTerminatedLocal
        );
        assert_eq!(
            DisconnectReason::from("org.bluez.Reason.Remote"),
            DisconnectReason::ConnectionTerminatedRemote
        );
        assert_eq!(
            DisconnectReason::from("org.bluez.Reason.Authentication"),
            DisconnectReason::AuthenticationFailure
        );
        assert_eq!(
            DisconnectReason::from("org.bluez.Reason.Suspend"),
            DisconnectReason::Suspend
        );
    }

    #[test]
    fn disconnect_reason_from_str_defaults_to_unknown() {
        assert_eq!(DisconnectReason::from("unknown"), DisconnectReason::Unknown);
        assert_eq!(DisconnectReason::from(""), DisconnectReason::Unknown);
    }

    #[test]
    fn disconnect_reason_from_u8_handles_hci_error_codes() {
        assert_eq!(
            DisconnectReason::from(0x08),
            DisconnectReason::ConnectionTimeout
        );
        assert_eq!(
            DisconnectReason::from(0x16),
            DisconnectReason::ConnectionTerminatedLocal
        );
        assert_eq!(
            DisconnectReason::from(0x13),
            DisconnectReason::ConnectionTerminatedRemote
        );
        assert_eq!(
            DisconnectReason::from(0x05),
            DisconnectReason::AuthenticationFailure
        );
    }

    #[test]
    fn disconnect_reason_from_u8_defaults_to_unknown() {
        assert_eq!(DisconnectReason::from(0xFF), DisconnectReason::Unknown);
        assert_eq!(DisconnectReason::from(0x00), DisconnectReason::Unknown);
        assert_eq!(DisconnectReason::from(0x42), DisconnectReason::Unknown);
    }
}
