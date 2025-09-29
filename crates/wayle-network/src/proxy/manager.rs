//! NetworkManager main D-Bus interface.

use std::collections::HashMap;

use zbus::{
    proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

/// Connection Manager.
///
/// The main NetworkManager D-Bus interface for managing network connections.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub(crate) trait NetworkManager {
    /// Reload NetworkManager's configuration and perform certain updates, like flushing a cache or rewriting external state to disk.
    ///
    /// This is similar to sending SIGHUP to NetworkManager but it allows for more fine-grained control over what to reload (see flags).
    /// It also allows non-root access via PolicyKit and contrary to signals it is synchronous.
    ///
    /// # Arguments
    /// * `flags` - Optional flags to specify which parts shall be reloaded:
    ///   - `0x00` - Reload everything (same as SIGHUP)
    ///   - `0x01` - Reload NetworkManager.conf from disk
    ///   - `0x02` - Update DNS configuration (rewrite /etc/resolv.conf)
    ///   - `0x04` - Restart DNS plugin
    fn reload(&self, flags: u32) -> zbus::Result<()>;

    /// Get the list of realized network devices.
    ///
    /// # Returns
    /// List of object paths of network devices known to the system. This list does not include device placeholders.
    fn get_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Get the list of all network devices.
    ///
    /// # Returns
    /// List of object paths of network devices and device placeholders (eg, devices that do not yet exist but which can be automatically created by NetworkManager if one of their AvailableConnections was activated).
    fn get_all_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Return the object path of the network device referenced by its IP interface name.
    ///
    /// Note that some devices (usually modems) only have an IP interface name when they are connected.
    ///
    /// # Arguments
    /// * `iface` - Interface name of the device to find
    ///
    /// # Returns
    /// Object path of the network device
    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<OwnedObjectPath>;

    /// Activate a connection using the supplied device.
    ///
    /// # Arguments
    /// * `connection` - The connection to activate. If "/" is given, a valid device path must be given, and NetworkManager picks the best connection to activate for the given device. VPN connections must always pass a valid connection path.
    /// * `device` - The object path of device to be activated for physical connections. This parameter is ignored for VPN connections, because the specific_object (if provided) specifies the device to use.
    /// * `specific_object` - The path of a connection-type-specific object this activation should use. This parameter is currently ignored for wired and mobile broadband connections, and the value of "/" should be used (ie, no specific object). For Wi-Fi connections, pass the object path of a specific AP from the card's scan list, or "/" to pick an AP automatically. For VPN connections, pass the object path of an ActiveConnection object that should serve as the "base" connection (to which the VPN connections lifetime will be tied), or pass "/" and NM will automatically use the current default device.
    ///
    /// # Returns
    /// The path of the active connection object representing this active connection.
    fn activate_connection(
        &self,
        connection: &OwnedObjectPath,
        device: &OwnedObjectPath,
        specific_object: &OwnedObjectPath,
    ) -> zbus::Result<OwnedObjectPath>;

    /// Adds a new connection using the given details (if any) as a template (automatically filling in missing settings with the capabilities of the given device and specific object), then activate the new connection.
    ///
    /// Cannot be used for VPN connections at this time.
    ///
    /// # Arguments
    /// * `connection` - Connection settings and properties; if incomplete missing settings will be automatically completed using the given device and specific object.
    /// * `device` - The object path of device to be activated using the given connection.
    /// * `specific_object` - The path of a connection-type-specific object this activation should use. This parameter is currently ignored for wired and mobile broadband connections, and the value of "/" should be used (ie, no specific object). For Wi-Fi connections, pass the object path of a specific AP from the card's scan list, which will be used to complete the details of the newly added connection.
    ///
    /// # Returns
    /// * Object path of the new connection that was just added
    /// * The path of the active connection object representing this active connection
    fn add_and_activate_connection(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        device: &OwnedObjectPath,
        specific_object: &OwnedObjectPath,
    ) -> zbus::Result<(OwnedObjectPath, OwnedObjectPath)>;

    /// Adds a new connection using the given details (if any) as a template, then activate the new connection.
    ///
    /// This method extends AddAndActivateConnection to allow passing further parameters.
    ///
    /// # Arguments
    /// * `connection` - Connection settings and properties
    /// * `device` - The object path of device to be activated
    /// * `specific_object` - The path of a connection-type-specific object
    /// * `options` - Further options:
    ///   - `persist`: "disk" (default), "memory", or "volatile"
    ///   - `bind-activation`: "dbus-client" or "none" (default)
    ///
    /// # Returns
    /// * Object path of the new connection
    /// * Path of the active connection
    /// * Additional results dictionary
    fn add_and_activate_connection2(
        &self,
        connection: HashMap<String, HashMap<String, OwnedValue>>,
        device: &OwnedObjectPath,
        specific_object: &OwnedObjectPath,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::Result<(
        OwnedObjectPath,
        OwnedObjectPath,
        HashMap<String, OwnedValue>,
    )>;

    /// Deactivate an active connection.
    ///
    /// # Arguments
    /// * `active_connection` - The currently active connection to deactivate
    fn deactivate_connection(&self, active_connection: &OwnedObjectPath) -> zbus::Result<()>;

    /// Control the NetworkManager daemon's sleep state.
    ///
    /// When asleep, all interfaces that it manages are deactivated. When awake, devices are available to be activated.
    /// This command should not be called directly by users or clients; it is intended for system suspend/resume tracking.
    ///
    /// # Arguments
    /// * `sleep` - Indicates whether the NetworkManager daemon should sleep or wake
    fn sleep(&self, sleep: bool) -> zbus::Result<()>;

    /// Control whether overall networking is enabled or disabled.
    ///
    /// When disabled, all interfaces that NM manages are deactivated. When enabled, all managed interfaces are re-enabled and available to be activated.
    /// This command should be used by clients that provide to users the ability to enable/disable all networking.
    ///
    /// # Arguments
    /// * `enable` - If FALSE, indicates that all networking should be disabled. If TRUE, indicates that NetworkManager should begin managing network devices.
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// Returns the permissions a caller has for various authenticated operations that NetworkManager provides.
    ///
    /// # Returns
    /// Dictionary of available permissions and results. Each permission is represented by a name (ie "org.freedesktop.NetworkManager.Foobar") and each result is one of the following values: "yes" (the permission is available), "auth" (the permission is available after a successful authentication), or "no" (the permission is denied).
    fn get_permissions(&self) -> zbus::Result<HashMap<String, String>>;

    /// Set logging verbosity and which operations are logged.
    ///
    /// # Arguments
    /// * `level` - One of [ERR, WARN, INFO, DEBUG, TRACE, OFF, KEEP]. This level is applied to the domains as specified in the domains argument. Except for the special level "KEEP", all unmentioned domains are disabled entirely. "KEEP" is special and allows not to change the current setting except for the specified domains.
    /// * `domains` - A combination of logging domains separated by commas (','), or "NONE" to disable logging. Each domain enables logging for operations related to that domain. Available domains are: [PLATFORM, RFKILL, ETHER, WIFI, BT, MB, DHCP4, DHCP6, PPP, WIFI_SCAN, IP4, IP6, AUTOIP4, DNS, VPN, SHARING, SUPPLICANT, AGENTS, SETTINGS, SUSPEND, CORE, DEVICE, OLPC, WIMAX, INFINIBAND, FIREWALL, ADSL, BOND, VLAN, BRIDGE, DBUS_PROPS, TEAM, CONCHECK, DCB, DISPATCH, AUDIT].
    fn set_logging(&self, level: &str, domains: &str) -> zbus::Result<()>;

    /// Get current logging verbosity level and operations domains.
    ///
    /// # Returns
    /// * Current log level: One of [ERR, WARN, INFO, DEBUG, TRACE]
    /// * Active log domains
    fn get_logging(&self) -> zbus::Result<(String, String)>;

    /// Re-check the network connectivity state.
    ///
    /// # Returns
    /// The current connectivity state
    fn check_connectivity(&self) -> zbus::Result<u32>;

    /// The overall networking state as determined by the NetworkManager daemon, based on the state of network devices under its management.
    ///
    /// # Returns
    /// Overall network state
    fn state(&self) -> zbus::Result<u32>;

    /// Create a checkpoint of the current networking configuration for given interfaces.
    ///
    /// If rollback_timeout is not zero, a rollback is automatically performed after the given timeout.
    ///
    /// # Arguments
    /// * `devices` - A list of device paths for which a checkpoint should be created. An empty list means all devices.
    /// * `rollback_timeout` - The time in seconds until NetworkManager will automatically rollback to the checkpoint. Set to zero for infinite.
    /// * `flags` - Flags for the creation (NMCheckpointCreateFlags)
    ///
    /// # Returns
    /// On success, the path of the new checkpoint
    fn checkpoint_create(
        &self,
        devices: Vec<OwnedObjectPath>,
        rollback_timeout: u32,
        flags: u32,
    ) -> zbus::Result<OwnedObjectPath>;

    /// Destroy a previously created checkpoint.
    ///
    /// # Arguments
    /// * `checkpoint` - The checkpoint to be destroyed. Set to empty to cancel all pending checkpoints.
    fn checkpoint_destroy(&self, checkpoint: &OwnedObjectPath) -> zbus::Result<()>;

    /// Rollback a checkpoint before the timeout is reached.
    ///
    /// # Arguments
    /// * `checkpoint` - The checkpoint to be rolled back
    ///
    /// # Returns
    /// Dictionary of devices and results. Devices are represented by their original D-Bus path; each result is a RollbackResult.
    fn checkpoint_rollback(
        &self,
        checkpoint: &OwnedObjectPath,
    ) -> zbus::Result<HashMap<String, u32>>;

    /// Reset the timeout for rollback for the checkpoint.
    ///
    /// # Arguments
    /// * `checkpoint` - The checkpoint to adjust
    /// * `add_timeout` - Number of seconds from now in which the timeout will expire. Set to 0 to disable the timeout.
    fn checkpoint_adjust_rollback_timeout(
        &self,
        checkpoint: &OwnedObjectPath,
        add_timeout: u32,
    ) -> zbus::Result<()>;

    /// The list of realized network devices.
    #[zbus(property)]
    fn devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The list of both realized and un-realized network devices.
    #[zbus(property)]
    fn all_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The list of active checkpoints.
    #[zbus(property)]
    fn checkpoints(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// Indicates if overall networking is currently enabled or not.
    #[zbus(property)]
    fn networking_enabled(&self) -> zbus::Result<bool>;

    /// Indicates if wireless is currently enabled or not.
    #[zbus(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
    /// Set if wireless should be enabled or not.
    #[zbus(property)]
    fn set_wireless_enabled(&self, enabled: bool) -> zbus::Result<()>;

    /// Indicates if the wireless hardware is currently enabled.
    #[zbus(property)]
    fn wireless_hardware_enabled(&self) -> zbus::Result<bool>;

    /// Indicates if mobile broadband devices are currently enabled or not.
    #[zbus(property)]
    fn wwan_enabled(&self) -> zbus::Result<bool>;
    /// Set if mobile broadband devices should be enabled or not.
    #[zbus(property)]
    fn set_wwan_enabled(&self, enabled: bool) -> zbus::Result<()>;

    /// Indicates if the mobile broadband hardware is currently enabled.
    #[zbus(property)]
    fn wwan_hardware_enabled(&self) -> zbus::Result<bool>;

    /// Flags related to radio devices.
    ///
    /// See NMRadioFlags for the list of flags supported.
    #[zbus(property)]
    fn radio_flags(&self) -> zbus::Result<u32>;

    /// List of active connection object paths.
    #[zbus(property)]
    fn active_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The object path of the "primary" active connection being used to access the network.
    #[zbus(property)]
    fn primary_connection(&self) -> zbus::Result<OwnedObjectPath>;

    /// The connection type of the "primary" active connection.
    #[zbus(property)]
    fn primary_connection_type(&self) -> zbus::Result<String>;

    /// Indicates whether the connectivity is metered.
    #[zbus(property)]
    fn metered(&self) -> zbus::Result<u32>;

    /// The object path of an active connection that is currently being activated.
    #[zbus(property)]
    fn activating_connection(&self) -> zbus::Result<OwnedObjectPath>;

    /// Indicates whether NM is still starting up.
    #[zbus(property)]
    fn startup(&self) -> zbus::Result<bool>;

    /// NetworkManager version.
    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;

    /// NetworkManager version and capabilities.
    ///
    /// The first element is the NM_VERSION (major << 16 | minor << 8 | micro).
    /// Following elements are a bitfield of static capabilities.
    #[zbus(property)]
    fn version_info(&self) -> zbus::Result<Vec<u32>>;

    /// The current set of capabilities.
    ///
    /// Array is guaranteed to be sorted in ascending order without duplicates.
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<Vec<u32>>;

    /// The overall state of the NetworkManager daemon.
    #[zbus(property, name = "State")]
    fn state_property(&self) -> zbus::Result<u32>;

    /// The result of the last connectivity check.
    #[zbus(property)]
    fn connectivity(&self) -> zbus::Result<u32>;

    /// Indicates whether connectivity checking service has been configured.
    #[zbus(property)]
    fn connectivity_check_available(&self) -> zbus::Result<bool>;

    /// Indicates whether connectivity checking is enabled.
    #[zbus(property)]
    fn connectivity_check_enabled(&self) -> zbus::Result<bool>;

    /// Set whether connectivity checking should be enabled.
    #[zbus(property)]
    fn set_connectivity_check_enabled(&self, enabled: bool) -> zbus::Result<()>;

    /// The URI that NetworkManager will hit to check if there is internet connectivity.
    #[zbus(property)]
    fn connectivity_check_uri(&self) -> zbus::Result<String>;

    /// Dictionary of global DNS settings.
    #[zbus(property)]
    fn global_dns_configuration(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    /// Set Dictionary of global DNS settings.
    #[zbus(property)]
    fn set_global_dns_configuration(&self, config: HashMap<String, OwnedValue>)
    -> zbus::Result<()>;

    /// Emitted when system authorization details change.
    #[zbus(signal)]
    fn check_permissions(&self) -> zbus::Result<()>;

    /// NetworkManager's state changed.
    #[zbus(signal)]
    fn state_changed(&self, state: u32) -> zbus::Result<()>;

    /// A device was added to the system.
    #[zbus(signal)]
    fn device_added(&self, device_path: OwnedObjectPath) -> zbus::Result<()>;

    /// A device was removed from the system.
    #[zbus(signal)]
    fn device_removed(&self, device_path: OwnedObjectPath) -> zbus::Result<()>;
}
