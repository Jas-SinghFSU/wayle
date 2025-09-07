//! NetworkManager PPP Device interface.

use zbus::proxy;

/// PPP Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Ppp"
)]
pub(crate) trait DevicePpp {
    // No properties
}
