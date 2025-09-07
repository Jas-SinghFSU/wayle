//! NetworkManager Access Point interface.

use zbus::proxy;

/// Wi-Fi Access Point.
///
/// Represents a visible WiFi access point.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.AccessPoint"
)]
pub(crate) trait AccessPoint {
    /// Flags describing the capabilities of the access point.
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// Flags describing the access point's capabilities according to WPA.
    #[zbus(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;

    /// Flags describing the access point's capabilities according to the RSN protocol.
    #[zbus(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;

    /// The Service Set Identifier identifying the access point.
    #[zbus(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    /// The radio channel frequency in use by the access point, in MHz.
    #[zbus(property)]
    fn frequency(&self) -> zbus::Result<u32>;

    /// The hardware address (BSSID) of the access point.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Describes the operating mode of the access point.
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// The maximum bitrate this access point is capable of, in kilobits/second (Kb/s).
    #[zbus(property)]
    fn max_bitrate(&self) -> zbus::Result<u32>;

    /// The current signal quality of the access point, in percent.
    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    /// The timestamp (in CLOCK_BOOTTIME seconds) for the last time the access point was found in scan results.
    #[zbus(property)]
    fn last_seen(&self) -> zbus::Result<i32>;
}
