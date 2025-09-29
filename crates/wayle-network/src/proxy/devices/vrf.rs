//! NetworkManager VRF Device interface.

use zbus::proxy;

/// VRF Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Vrf"
)]
pub(crate) trait DeviceVrf {
    /// The routing table ID.
    #[zbus(property)]
    fn table(&self) -> zbus::Result<u32>;
}
