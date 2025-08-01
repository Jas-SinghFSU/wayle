use std::fmt::{self, Display};

use crate::services::network_manager::{NM80211ApFlags, NM80211ApSecurityFlags};

/// Network identifier for SSIDs and BSSIDs.
///
/// Wraps raw bytes since 802.11 allows non-UTF8 identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkIdentifier(Vec<u8>);

/// Service Set Identifier - the network name.
pub type SSID = NetworkIdentifier;

/// Basic Service Set Identifier - the hardware address.
pub type BSSID = NetworkIdentifier;

impl NetworkIdentifier {
    /// Creates a new identifier from raw bytes.
    ///
    /// SSIDs are typically UTF-8 strings but 802.11 allows
    /// arbitrary byte sequences up to 32 octets.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns the identifier as a UTF-8 string.
    ///
    /// Invalid UTF-8 sequences are replaced with �.
    pub fn as_str(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }

    /// Returns the raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Checks if this is empty (hidden network for SSIDs).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for NetworkIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Vec<u8>> for NetworkIdentifier {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl From<String> for NetworkIdentifier {
    fn from(s: String) -> Self {
        Self::new(s.into_bytes())
    }
}

impl From<&str> for NetworkIdentifier {
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

/// Security type classification for access points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityType {
    /// No security (open network).
    None,
    /// WEP (Wired Equivalent Privacy) - deprecated and insecure.
    WEP,
    /// WPA (WiFi Protected Access) version 1.
    WPA,
    /// WPA2 (WiFi Protected Access) version 2 - most common.
    WPA2,
    /// WPA3 (WiFi Protected Access) version 3 - latest standard.
    WPA3,
    /// Enterprise security (802.1X) - requires authentication server.
    Enterprise,
}

impl SecurityType {
    /// Returns a human-readable string representation of the security type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "Open",
            Self::WEP => "WEP",
            Self::WPA => "WPA",
            Self::WPA2 => "WPA2",
            Self::WPA3 => "WPA3",
            Self::Enterprise => "Enterprise",
        }
    }

    /// Derive security type from AP flags.
    ///
    /// Analyzes WPA and RSN flags to determine the highest
    /// level of security supported by the access point.
    pub fn from_flags(
        flags: NM80211ApFlags,
        wpa_flags: NM80211ApSecurityFlags,
        rsn_flags: NM80211ApSecurityFlags,
    ) -> Self {
        if rsn_flags.contains(NM80211ApSecurityFlags::KEY_MGMT_SAE) {
            SecurityType::WPA3
        } else if rsn_flags.contains(NM80211ApSecurityFlags::KEY_MGMT_802_1X)
            || wpa_flags.contains(NM80211ApSecurityFlags::KEY_MGMT_802_1X)
        {
            SecurityType::Enterprise
        } else if rsn_flags != NM80211ApSecurityFlags::NONE {
            SecurityType::WPA2
        } else if wpa_flags != NM80211ApSecurityFlags::NONE {
            SecurityType::WPA
        } else if flags.contains(NM80211ApFlags::PRIVACY) {
            SecurityType::WEP
        } else {
            SecurityType::None
        }
    }
}

impl Display for SecurityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
