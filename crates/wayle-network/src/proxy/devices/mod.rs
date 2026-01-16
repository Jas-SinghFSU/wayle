//! NetworkManager Device interfaces.

use std::collections::HashMap;

use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

pub mod adsl;
pub mod bluetooth;
pub mod bond;
pub mod bridge;
pub mod dummy;
pub mod generic;
pub mod hsr;
pub mod infiniband;
pub mod ip_tunnel;
pub mod ipvlan;
pub mod loopback;
pub mod lowpan;
pub mod macsec;
pub mod macvlan;
pub mod modem;
pub mod olpc_mesh;
pub mod ovs_bridge;
pub mod ovs_interface;
pub mod ovs_port;
pub mod ppp;
pub mod statistics;
pub mod team;
pub mod tun;
pub mod veth;
pub mod vlan;
pub mod vrf;
pub mod vxlan;
pub mod wifi_p2p;
pub mod wired;
pub mod wireguard;
pub mod wireless;
pub mod wpan;

type NMAppliedConnection = HashMap<String, HashMap<String, OwnedValue>>;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device"
)]
pub(crate) trait Device {
    /// Attempts to update device with new connection settings and properties.
    ///
    /// # Arguments
    /// * `connection` - Optional connection settings
    /// * `version_id` - Settings version id (0 for current)
    /// * `flags` - Flags (none defined)
    fn reapply(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        version_id: u64,
        flags: u32,
    ) -> zbus::Result<()>;

    /// Get the currently applied connection on the device.
    ///
    /// # Arguments
    /// * `flags` - Flags (none defined)
    ///
    /// # Returns
    /// * Connection settings
    /// * Version id
    fn get_applied_connection(&self, flags: u32) -> zbus::Result<(NMAppliedConnection, u64)>;

    /// Disconnects a device and prevents the device from automatically activating further connections without user intervention.
    fn disconnect(&self) -> zbus::Result<()>;

    /// Deletes a software device from NetworkManager and removes the interface from the system.
    fn delete(&self) -> zbus::Result<()>;

    /// Operating-system specific transient device hardware identifier.
    #[zbus(property)]
    fn udi(&self) -> zbus::Result<String>;

    /// The path of the device as exposed by the udev property ID_PATH.
    #[zbus(property)]
    fn path(&self) -> zbus::Result<String>;

    /// The name of the device's control (and often data) interface.
    #[zbus(property)]
    fn interface(&self) -> zbus::Result<String>;

    /// The name of the device's data interface when available.
    #[zbus(property)]
    fn ip_interface(&self) -> zbus::Result<String>;

    /// The driver handling the device.
    #[zbus(property)]
    fn driver(&self) -> zbus::Result<String>;

    /// The version of the driver handling the device.
    #[zbus(property)]
    fn driver_version(&self) -> zbus::Result<String>;

    /// The firmware version for the device.
    #[zbus(property)]
    fn firmware_version(&self) -> zbus::Result<String>;

    /// Flags describing the capabilities of the device.
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<u32>;

    /// The current state of the device.
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// The current state and reason for that state.
    #[zbus(property)]
    fn state_reason(&self) -> zbus::Result<(u32, u32)>;

    /// Object path of an ActiveConnection object that "owns" this device during activation.
    #[zbus(property)]
    fn active_connection(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Ip4Config object describing the configuration of the device.
    #[zbus(property)]
    fn ip4_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Dhcp4Config object describing the DHCP options returned by the DHCP server.
    #[zbus(property)]
    fn dhcp4_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Ip6Config object describing the configuration of the device.
    #[zbus(property)]
    fn ip6_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Object path of the Dhcp6Config object describing the DHCP options returned by the DHCP server.
    #[zbus(property)]
    fn dhcp6_config(&self) -> zbus::Result<OwnedObjectPath>;

    /// Whether or not this device is managed by NetworkManager.
    #[zbus(property)]
    fn managed(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_managed(&self, managed: bool) -> zbus::Result<()>;

    /// If TRUE, indicates the device is allowed to autoconnect.
    #[zbus(property)]
    fn autoconnect(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_autoconnect(&self, autoconnect: bool) -> zbus::Result<()>;

    /// If TRUE, indicates the device is likely missing firmware necessary for its operation.
    #[zbus(property)]
    fn firmware_missing(&self) -> zbus::Result<bool>;

    /// If TRUE, indicates the NetworkManager plugin for the device is likely missing or misconfigured.
    #[zbus(property)]
    fn nm_plugin_missing(&self) -> zbus::Result<bool>;

    /// The general type of the network device.
    #[zbus(property)]
    fn device_type(&self) -> zbus::Result<u32>;

    /// An array of object paths of every configured connection that is currently 'available' through this device.
    #[zbus(property)]
    fn available_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// If non-empty, an (opaque) indicator of the physical network port associated with the device.
    #[zbus(property)]
    fn physical_port_id(&self) -> zbus::Result<String>;

    /// The MTU of the device.
    #[zbus(property)]
    fn mtu(&self) -> zbus::Result<u32>;

    /// Whether the amount of traffic flowing through the device is subject to limitations.
    #[zbus(property)]
    fn metered(&self) -> zbus::Result<u32>;

    /// Array of LLDP neighbors.
    #[zbus(property)]
    fn lldp_neighbors(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// True if the device exists, or False for placeholder devices.
    #[zbus(property)]
    fn real(&self) -> zbus::Result<bool>;

    /// The result of the last IPv4 connectivity check.
    #[zbus(property)]
    fn ip4_connectivity(&self) -> zbus::Result<u32>;

    /// The result of the last IPv6 connectivity check.
    #[zbus(property)]
    fn ip6_connectivity(&self) -> zbus::Result<u32>;

    /// The flags of the network interface.
    #[zbus(property)]
    fn interface_flags(&self) -> zbus::Result<u32>;

    /// The hardware address of the device.
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// The port devices of the controller device.
    #[zbus(property)]
    fn ports(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Emitted when the device's state changes.
    #[zbus(signal, name = "StateChanged")]
    fn device_state_changed(&self, new_state: u32, old_state: u32, reason: u32)
    -> zbus::Result<()>;
}
