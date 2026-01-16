//! NetworkManager PPP Device interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Ppp"
)]
pub(crate) trait DevicePpp {
    // No properties
}
