//! NetworkManager MACsec Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// MACsec Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Macsec"
)]
pub(crate) trait DeviceMacsec {
    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The Secure Channel Identifier.
    #[zbus(property)]
    fn sci(&self) -> zbus::Result<u64>;

    /// The size of the Integrity Check Value.
    #[zbus(property)]
    fn icv_length(&self) -> zbus::Result<u8>;

    /// The set of cryptographic algorithms in use.
    #[zbus(property)]
    fn cipher_suite(&self) -> zbus::Result<u64>;

    /// The length of the replay window.
    #[zbus(property)]
    fn window(&self) -> zbus::Result<u32>;

    /// The transmission encoding mode.
    #[zbus(property)]
    fn encoding_sa(&self) -> zbus::Result<u8>;

    /// The validation mode.
    #[zbus(property)]
    fn validation(&self) -> zbus::Result<String>;

    /// Whether encryption of transmitted frames is enabled.
    #[zbus(property)]
    fn encrypt(&self) -> zbus::Result<bool>;

    /// Whether the SCI is always included in transmitted SecTAG.
    #[zbus(property)]
    fn protect(&self) -> zbus::Result<bool>;

    /// Whether the SCI is always included in transmitted SecTAG.
    #[zbus(property)]
    fn include_sci(&self) -> zbus::Result<bool>;

    /// Whether the ES (End Station) bit is set in transmitted SecTAG.
    #[zbus(property)]
    fn es(&self) -> zbus::Result<bool>;

    /// Whether the SCB (Single Copy Broadcast) bit is set in transmitted SecTAG.
    #[zbus(property)]
    fn scb(&self) -> zbus::Result<bool>;

    /// Whether replay protection is enabled.
    #[zbus(property)]
    fn replay_protect(&self) -> zbus::Result<bool>;
}
