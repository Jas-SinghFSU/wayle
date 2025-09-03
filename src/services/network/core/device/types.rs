use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct DeviceParams<'a> {
    pub connection: &'a Connection,
    pub object_path: OwnedObjectPath,
}

pub(crate) struct LiveDeviceParams<'a> {
    pub connection: &'a Connection,
    pub object_path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}

/// Fetched device properties from D-Bus
pub(crate) struct DeviceProperties {
    pub udi: String,
    pub udev_path: String,
    pub interface: String,
    pub ip_interface: String,
    pub driver: String,
    pub driver_version: String,
    pub firmware_version: String,
    pub capabilities: u32,
    pub state: u32,
    pub state_reason: (u32, u32),
    pub active_connection: OwnedObjectPath,
    pub ip4_config: OwnedObjectPath,
    pub dhcp4_config: OwnedObjectPath,
    pub ip6_config: OwnedObjectPath,
    pub dhcp6_config: OwnedObjectPath,
    pub managed: bool,
    pub autoconnect: bool,
    pub firmware_missing: bool,
    pub nm_plugin_missing: bool,
    pub device_type: u32,
    pub available_connections: Vec<OwnedObjectPath>,
    pub physical_port_id: String,
    pub mtu: u32,
    pub metered: u32,
    pub real: bool,
    pub ip4_connectivity: u32,
    pub ip6_connectivity: u32,
    pub interface_flags: u32,
    pub hw_address: String,
    pub ports: Vec<OwnedObjectPath>,
}
