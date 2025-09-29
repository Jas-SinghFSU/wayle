//! NetworkManager Wi-Fi types.

/// Indicates the 802.11 mode an access point or device is currently in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NM80211Mode {
    /// the device or access point mode is unknown
    Unknown = 0,
    /// for both devices and access point objects, indicates the object is part of an
    /// Ad-Hoc 802.11 network without a central coordinating access point.
    Adhoc = 1,
    /// the device or access point is in infrastructure mode. For devices, this indicates
    /// the device is an 802.11 client/station. For access point objects, this indicates
    /// the object is an access point that provides connectivity to clients.
    Infra = 2,
    /// the device is an access point/hotspot. Not valid for access point objects; used
    /// only for hotspot mode on the local machine.
    Ap = 3,
    /// the device is a 802.11s mesh point. Since: 1.20.
    Mesh = 4,
}

impl NM80211Mode {
    /// Convert from D-Bus u32 representation
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Adhoc,
            2 => Self::Infra,
            3 => Self::Ap,
            4 => Self::Mesh,
            _ => Self::Unknown,
        }
    }
}
