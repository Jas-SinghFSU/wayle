/// Device control operations
mod controls;
/// Device monitoring implementation
mod monitoring;
/// WiFi device functionality and management.
pub mod wifi;
/// Wired (ethernet) device functionality and management.
pub mod wired;

use std::{collections::HashMap, sync::Arc};

use controls::DeviceControls;
use monitoring::DeviceMonitor;
use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::{
    services::{
        common::Property,
        network::{
            LldpNeighbor, NMConnectivityState, NMDeviceCapabilities, NMDeviceInterfaceFlags,
            NMDeviceState, NMDeviceStateReason, NMDeviceType, NMMetered, NetworkError,
            proxy::devices::DeviceProxy,
        },
    },
    unwrap_bool, unwrap_bool_or, unwrap_path, unwrap_string, unwrap_u32, unwrap_u32_or, unwrap_vec,
};

/// Wrapper around zbus::Connection that implements Debug.
#[derive(Clone)]
pub(crate) struct DbusConnection(pub Connection);

impl std::fmt::Debug for DbusConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection").finish()
    }
}

impl std::ops::Deref for DbusConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Network device managed by NetworkManager.
///
/// Common functionality for all network interfaces (WiFi, ethernet, etc).
/// Contains hardware information, state, configuration, and statistics.
#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) connection: DbusConnection,

    /// D-Bus object path for this device.
    pub object_path: OwnedObjectPath,

    /// Operating-system specific transient device hardware identifier. Opaque
    /// string representing the underlying hardware for the device, and shouldn't be used to
    /// keep track of individual devices. For some device types (Bluetooth, Modems) it is an
    /// identifier used by the hardware service (eg bluez or ModemManager) to refer to that
    /// device, and client programs use it get additional information from those services
    /// which NM does not provide. The Udi is not guaranteed to be consistent across reboots
    /// or hotplugs of the hardware.
    pub udi: Property<String>,

    /// The path of the device as exposed by the udev property ID_PATH.
    pub udev_path: Property<String>,

    /// The name of the device's control (and often data) interface. Note that non UTF-8
    /// characters are backslash escaped, so the resulting name may be longer then 15
    /// characters. Use g_strcompress() to revert the escaping.
    pub interface: Property<String>,

    /// The name of the device's data interface when available. May not refer
    /// to the actual data interface until the device has successfully established a data
    /// connection, indicated by the device's State becoming ACTIVATED. Note that non UTF-8
    /// characters are backslash escaped, so the resulting name may be longer then 15
    /// characters. Use g_strcompress() to revert the escaping.
    pub ip_interface: Property<String>,

    /// The driver handling the device. Non-UTF-8 sequences are backslash escaped.
    pub driver: Property<String>,

    /// The version of the driver handling the device. Non-UTF-8 sequences are backslash
    /// escaped.
    pub driver_version: Property<String>,

    /// The firmware version for the device. Non-UTF-8 sequences are backslash escaped.
    pub firmware_version: Property<String>,

    /// Flags describing the capabilities of the device. See NMDeviceCapabilities.
    pub capabilities: Property<NMDeviceCapabilities>,

    /// The current state of the device.
    pub state: Property<NMDeviceState>,

    /// The current state and reason for that state.
    pub state_reason: Property<(NMDeviceState, NMDeviceStateReason)>,

    /// Object path of an ActiveConnection object that "owns" this device during activation.
    /// The ActiveConnection object tracks the life-cycle of a connection to a specific
    /// network and implements the org.freedesktop.NetworkManager.Connection.Active D-Bus
    /// interface.
    pub active_connection: Property<OwnedObjectPath>,

    /// Object path of the Ip4Config object describing the configuration of the device. Only
    /// valid when the device is in the NM_DEVICE_STATE_ACTIVATED state.
    pub ip4_config: Property<OwnedObjectPath>,

    /// Object path of the Dhcp4Config object describing the DHCP options returned by the
    /// DHCP server. Only valid when the device is in the NM_DEVICE_STATE_ACTIVATED state.
    pub dhcp4_config: Property<OwnedObjectPath>,

    /// Object path of the Ip6Config object describing the configuration of the device. Only
    /// valid when the device is in the NM_DEVICE_STATE_ACTIVATED state.
    pub ip6_config: Property<OwnedObjectPath>,

    /// Object path of the Dhcp6Config object describing the DHCP options returned by the
    /// DHCP server. Only valid when the device is in the NM_DEVICE_STATE_ACTIVATED state.
    pub dhcp6_config: Property<OwnedObjectPath>,

    /// Whether or not this device is managed by NetworkManager. Setting this property has a
    /// similar effect to configuring the device as unmanaged via the
    /// keyfile.unmanaged-devices setting in NetworkManager.conf.
    pub managed: Property<bool>,

    /// If TRUE, indicates the device is allowed to autoconnect. If FALSE, manual
    /// intervention is required before the device will automatically connect to a known
    /// network, such as activating a connection using the device, or setting this property
    /// to TRUE.
    pub autoconnect: Property<bool>,

    /// If TRUE, indicates the device is likely missing firmware necessary for its
    /// operation.
    pub firmware_missing: Property<bool>,

    /// If TRUE, indicates the NetworkManager plugin for the device is likely missing or
    /// misconfigured.
    pub nm_plugin_missing: Property<bool>,

    /// The general type of the network device.
    pub device_type: Property<NMDeviceType>,

    /// An array of object paths of every configured connection that is currently 'available'
    /// through this device.
    pub available_connections: Property<Vec<OwnedObjectPath>>,

    /// If non-empty, an (opaque) indicator of the physical network port associated with the
    /// device. Can be used to recognize when two seemingly-separate hardware devices
    /// are actually just different virtual interfaces to the same physical port.
    pub physical_port_id: Property<String>,

    /// The MTU of the device.
    pub mtu: Property<u32>,

    /// Whether the amount of traffic flowing through the device is subject to limitations,
    /// for example set by service providers.
    pub metered: Property<NMMetered>,

    /// Array of LLDP neighbors; each element is a dictionary mapping LLDP TLV names to
    /// variant boxed values.
    pub lldp_neighbors: Property<Vec<LldpNeighbor>>,

    /// True if the device exists, or False for placeholder devices that do not yet exist but
    /// could be automatically created by NetworkManager if one of their
    /// AvailableConnections was activated.
    pub real: Property<bool>,

    /// The result of the last IPv4 connectivity check.
    pub ip4_connectivity: Property<NMConnectivityState>,

    /// The result of the last IPv6 connectivity check.
    pub ip6_connectivity: Property<NMConnectivityState>,

    /// The flags of the network interface. See NMDeviceInterfaceFlags for the currently
    /// defined flags.
    pub interface_flags: Property<NMDeviceInterfaceFlags>,

    /// The hardware address of the device.
    pub hw_address: Property<String>,

    /// The port devices of the controller device. Array of object paths of port devices for
    /// controller devices. For devices that are not controllers this is an empty array.
    pub ports: Property<Vec<OwnedObjectPath>>,
}

/// Fetched device properties from D-Bus
struct DeviceProperties {
    udi: String,
    udev_path: String,
    interface: String,
    ip_interface: String,
    driver: String,
    driver_version: String,
    firmware_version: String,
    capabilities: u32,
    state: u32,
    state_reason: (u32, u32),
    active_connection: OwnedObjectPath,
    ip4_config: OwnedObjectPath,
    dhcp4_config: OwnedObjectPath,
    ip6_config: OwnedObjectPath,
    dhcp6_config: OwnedObjectPath,
    managed: bool,
    autoconnect: bool,
    firmware_missing: bool,
    nm_plugin_missing: bool,
    device_type: u32,
    available_connections: Vec<OwnedObjectPath>,
    physical_port_id: String,
    mtu: u32,
    metered: u32,
    real: bool,
    ip4_connectivity: u32,
    ip6_connectivity: u32,
    interface_flags: u32,
    hw_address: String,
    ports: Vec<OwnedObjectPath>,
}

impl Device {
    pub(crate) async fn get(
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let device = Self::from_path(connection, object_path).await?;
        Ok(Arc::new(device))
    }

    pub(crate) async fn get_live(
        connection: &Connection,
        object_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, NetworkError> {
        let device = Self::from_path(connection, object_path.clone())
            .await
            .map_err(|e| NetworkError::ObjectCreationFailed {
                object_type: "Device".to_string(),
                object_path: object_path.clone(),
                reason: e.to_string(),
            })?;

        let device = Arc::new(device);
        DeviceMonitor::start(device.clone(), connection, object_path, cancellation_token).await?;

        Ok(device)
    }

    pub(crate) async fn from_path(
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Result<Self, NetworkError> {
        let proxy = DeviceProxy::new(connection, object_path.clone()).await?;
        let props = Self::fetch_properties(&proxy).await?;
        Ok(Self::from_properties(props, connection, object_path))
    }

    #[allow(clippy::too_many_lines)]
    async fn fetch_properties(proxy: &DeviceProxy<'_>) -> Result<DeviceProperties, NetworkError> {
        let (udi, path, interface, ip_interface, driver, driver_version, firmware_version) = tokio::join!(
            proxy.udi(),
            proxy.path(),
            proxy.interface(),
            proxy.ip_interface(),
            proxy.driver(),
            proxy.driver_version(),
            proxy.firmware_version(),
        );

        let (
            capabilities,
            state,
            state_reason,
            active_connection,
            ip4_config,
            dhcp4_config,
            ip6_config,
            dhcp6_config,
        ) = tokio::join!(
            proxy.capabilities(),
            proxy.state(),
            proxy.state_reason(),
            proxy.active_connection(),
            proxy.ip4_config(),
            proxy.dhcp4_config(),
            proxy.ip6_config(),
            proxy.dhcp6_config(),
        );

        let (
            managed,
            autoconnect,
            firmware_missing,
            nm_plugin_missing,
            device_type,
            available_connections,
            physical_port_id,
            mtu,
        ) = tokio::join!(
            proxy.managed(),
            proxy.autoconnect(),
            proxy.firmware_missing(),
            proxy.nm_plugin_missing(),
            proxy.device_type(),
            proxy.available_connections(),
            proxy.physical_port_id(),
            proxy.mtu(),
        );

        let (
            metered,
            real,
            ip4_connectivity,
            ip6_connectivity,
            interface_flags,
            hw_address,
            ports,
            _lldp_neighbors,
        ) = tokio::join!(
            proxy.metered(),
            proxy.real(),
            proxy.ip4_connectivity(),
            proxy.ip6_connectivity(),
            proxy.interface_flags(),
            proxy.hw_address(),
            proxy.ports(),
            proxy.lldp_neighbors(),
        );

        let device_path = path.clone().unwrap_or_default();

        let available_connections: Vec<OwnedObjectPath> =
            unwrap_vec!(available_connections, device_path)
                .into_iter()
                .map(|p| OwnedObjectPath::try_from(p.to_string()).unwrap_or_default())
                .collect();

        let ports: Vec<OwnedObjectPath> = unwrap_vec!(ports, device_path)
            .into_iter()
            .map(|p| OwnedObjectPath::try_from(p.to_string()).unwrap_or_default())
            .collect();

        Ok(DeviceProperties {
            udi: unwrap_string!(udi, device_path),
            interface: unwrap_string!(interface, device_path),
            ip_interface: unwrap_string!(ip_interface, device_path),
            driver: unwrap_string!(driver, device_path),
            driver_version: unwrap_string!(driver_version, device_path),
            firmware_version: unwrap_string!(firmware_version, device_path),
            capabilities: unwrap_u32!(capabilities, device_path),
            state: unwrap_u32!(state, device_path),
            state_reason: state_reason.unwrap_or((0, 0)),
            active_connection: unwrap_path!(active_connection, device_path),
            ip4_config: unwrap_path!(ip4_config, device_path),
            dhcp4_config: unwrap_path!(dhcp4_config, device_path),
            ip6_config: unwrap_path!(ip6_config, device_path),
            dhcp6_config: unwrap_path!(dhcp6_config, device_path),
            managed: unwrap_bool_or!(managed, device_path, true),
            autoconnect: unwrap_bool!(autoconnect, device_path),
            firmware_missing: unwrap_bool!(firmware_missing, device_path),
            nm_plugin_missing: unwrap_bool!(nm_plugin_missing, device_path),
            device_type: unwrap_u32!(device_type, device_path),
            available_connections,
            physical_port_id: unwrap_string!(physical_port_id, device_path),
            mtu: unwrap_u32_or!(mtu, device_path, 1500),
            metered: unwrap_u32!(metered, device_path),
            real: unwrap_bool_or!(real, device_path, true),
            ip4_connectivity: unwrap_u32!(ip4_connectivity, device_path),
            ip6_connectivity: unwrap_u32!(ip6_connectivity, device_path),
            interface_flags: unwrap_u32!(interface_flags, device_path),
            hw_address: unwrap_string!(hw_address, device_path),
            ports,
            udev_path: device_path,
        })
    }

    fn from_properties(
        props: DeviceProperties,
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Self {
        Self {
            connection: DbusConnection(connection.clone()),
            object_path,
            udi: Property::new(props.udi),
            udev_path: Property::new(props.udev_path),
            interface: Property::new(props.interface),
            ip_interface: Property::new(props.ip_interface),
            driver: Property::new(props.driver),
            driver_version: Property::new(props.driver_version),
            firmware_version: Property::new(props.firmware_version),
            capabilities: Property::new(NMDeviceCapabilities::from_bits_truncate(
                props.capabilities,
            )),
            state: Property::new(NMDeviceState::from_u32(props.state)),
            state_reason: Property::new((
                NMDeviceState::from_u32(props.state_reason.0),
                NMDeviceStateReason::from_u32(props.state_reason.1),
            )),
            active_connection: Property::new(props.active_connection),
            ip4_config: Property::new(props.ip4_config),
            dhcp4_config: Property::new(props.dhcp4_config),
            ip6_config: Property::new(props.ip6_config),
            dhcp6_config: Property::new(props.dhcp6_config),
            managed: Property::new(props.managed),
            autoconnect: Property::new(props.autoconnect),
            firmware_missing: Property::new(props.firmware_missing),
            nm_plugin_missing: Property::new(props.nm_plugin_missing),
            device_type: Property::new(NMDeviceType::from_u32(props.device_type)),
            available_connections: Property::new(props.available_connections),
            physical_port_id: Property::new(props.physical_port_id),
            mtu: Property::new(props.mtu),
            metered: Property::new(NMMetered::from_u32(props.metered)),
            real: Property::new(props.real),
            ip4_connectivity: Property::new(NMConnectivityState::from_u32(props.ip4_connectivity)),
            ip6_connectivity: Property::new(NMConnectivityState::from_u32(props.ip6_connectivity)),
            interface_flags: Property::new(NMDeviceInterfaceFlags::from_bits_truncate(
                props.interface_flags,
            )),
            hw_address: Property::new(props.hw_address),
            ports: Property::new(props.ports),
            // No idea what the properties for LLDP are - feel free to open a PR if you need this
            // In the meantime, lldp_neighbors will always return an empty vec
            lldp_neighbors: Property::new(vec![]),
        }
    }

    /// Whether or not this device is managed by NetworkManager.
    ///
    /// # Errors
    /// Returns error if the D-Bus operation fails.
    pub async fn set_managed(&self, managed: bool) -> Result<(), NetworkError> {
        DeviceControls::set_managed(&self.connection, &self.object_path, managed).await
    }

    /// If TRUE, indicates the device is allowed to autoconnect.
    ///
    /// # Errors
    /// Returns error if the D-Bus operation fails.
    pub async fn set_autoconnect(&self, autoconnect: bool) -> Result<(), NetworkError> {
        DeviceControls::set_autoconnect(&self.connection, &self.object_path, autoconnect).await
    }

    /// Attempts to update device with new connection settings and properties.
    ///
    /// # Arguments
    /// * `connection` - Optional connection settings
    /// * `version_id` - Settings version id (0 for current)
    /// * `flags` - Flags (none defined)
    ///
    /// # Errors
    /// Returns error if the reapply operation fails.
    pub async fn reapply(
        &self,
        connection_settings: HashMap<String, HashMap<String, OwnedValue>>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), NetworkError> {
        DeviceControls::reapply(
            &self.connection,
            &self.object_path,
            connection_settings,
            version_id,
            flags,
        )
        .await
    }

    /// Get the currently applied connection on the device.
    ///
    /// # Arguments
    /// * `flags` - Flags (none defined)
    ///
    /// # Returns
    /// * Connection settings
    /// * Version id
    ///
    /// # Errors
    /// Returns error if getting the applied connection fails.
    pub async fn get_applied_connection(
        &self,
        flags: u32,
    ) -> Result<(HashMap<String, HashMap<String, OwnedValue>>, u64), NetworkError> {
        DeviceControls::get_applied_connection(&self.connection, &self.object_path, flags).await
    }

    /// Disconnects a device and prevents the device from automatically activating further connections without user intervention.
    ///
    /// # Errors
    /// Returns error if the disconnect operation fails.
    pub async fn disconnect(&self) -> Result<(), NetworkError> {
        DeviceControls::disconnect(&self.connection, &self.object_path).await
    }

    /// Deletes a software device from NetworkManager and removes the interface from the system.
    ///
    /// # Errors
    /// Returns error if the delete operation fails.
    pub async fn delete(&self) -> Result<(), NetworkError> {
        DeviceControls::delete(&self.connection, &self.object_path).await
    }
}
