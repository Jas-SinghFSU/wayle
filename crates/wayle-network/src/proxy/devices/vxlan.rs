//! NetworkManager VXLAN Device interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// VXLAN Device.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Vxlan"
)]
pub(crate) trait DeviceVxlan {
    /// Hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The object path of the parent device.
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<OwnedObjectPath>;

    /// The VXLAN Network Identifier (VNI).
    #[zbus(property)]
    fn id(&self) -> zbus::Result<u32>;

    /// The multicast IP (v4 or v6) joined.
    #[zbus(property)]
    fn group(&self) -> zbus::Result<String>;

    /// The local IP (v4 or v6) used to send multicast packets.
    #[zbus(property)]
    fn local(&self) -> zbus::Result<String>;

    /// The IP TTL.
    #[zbus(property)]
    fn ttl(&self) -> zbus::Result<u8>;

    /// The TOS field to set in outgoing packets.
    #[zbus(property)]
    fn tos(&self) -> zbus::Result<u8>;

    /// Whether ARP proxy is turned on.
    #[zbus(property)]
    fn proxy(&self) -> zbus::Result<bool>;

    /// Whether route short circuit is turned on.
    #[zbus(property)]
    fn rsc(&self) -> zbus::Result<bool>;

    /// Whether netlink LL ADDR miss notifications are generated.
    #[zbus(property)]
    fn l2miss(&self) -> zbus::Result<bool>;

    /// Whether netlink IP ADDR miss notifications are generated.
    #[zbus(property)]
    fn l3miss(&self) -> zbus::Result<bool>;

    /// Destination port for outgoing packets.
    #[zbus(property)]
    fn dst_port(&self) -> zbus::Result<u16>;

    /// The minimum source port for outgoing packets.
    #[zbus(property)]
    fn src_port_min(&self) -> zbus::Result<u16>;

    /// The maximum source port for outgoing packets.
    #[zbus(property)]
    fn src_port_max(&self) -> zbus::Result<u16>;

    /// Lifetime in seconds of FDB entries learnt by the kernel.
    #[zbus(property)]
    fn ageing(&self) -> zbus::Result<u32>;

    /// Maximum number of entries that can be added to the FDB.
    #[zbus(property)]
    fn limit(&self) -> zbus::Result<u32>;

    /// Whether netlink LL ADDR miss notifications are generated.
    #[zbus(property)]
    fn learning(&self) -> zbus::Result<bool>;
}
