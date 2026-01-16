//! NetworkManager Device Statistics interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Statistics"
)]
pub(crate) trait DeviceStatistics {
    /// Refresh rate of the rest of properties of this interface, in milliseconds.
    #[zbus(property)]
    fn refresh_rate_ms(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn set_refresh_rate_ms(&self, rate: u32) -> zbus::Result<()>;

    /// Number of transmitted bytes.
    #[zbus(property)]
    fn tx_bytes(&self) -> zbus::Result<u64>;

    /// Number of received bytes.
    #[zbus(property)]
    fn rx_bytes(&self) -> zbus::Result<u64>;
}
