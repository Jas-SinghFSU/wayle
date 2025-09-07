use std::collections::HashMap;

use zbus::{
    Result, proxy,
    zvariant::{ObjectPath, OwnedObjectPath, Value},
};

/// Proxy for the org.bluez.Adapter1 interface.
///
/// BlueZ D-Bus Adapter API for managing Bluetooth adapters.
///
/// Service: `org.bluez`
/// Interface: `org.bluez.Adapter1`
/// Object path: `[variable prefix]/{hci0,hci1,...}`
#[proxy(interface = "org.bluez.Adapter1", default_service = "org.bluez")]
pub(crate) trait Adapter1 {
    /// Starts device discovery session which may include starting an inquiry and/or
    /// scanning procedures and remote device name resolving.
    ///
    /// Use StopDiscovery to release the sessions acquired.
    ///
    /// This process will start creating Device objects as new devices are discovered.
    ///
    /// During discovery RSSI delta-threshold is imposed.
    ///
    /// Each client can request a single device discovery session per adapter.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    /// - `InProgress` - Discovery already in progress
    async fn start_discovery(&self) -> Result<()>;

    /// Stops device discovery session started by StartDiscovery.
    ///
    /// Note that a discovery procedure is shared between all discovery sessions thus
    /// calling StopDiscovery will only release a single session and discovery will stop
    /// when all sessions from all clients have finished.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    /// - `NotAuthorized` - Not authorized to stop discovery
    async fn stop_discovery(&self) -> Result<()>;

    /// Removes the remote device object at the given path including cached information
    /// such as bonding information.
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid device path
    /// - `Failed` - Operation failed
    async fn remove_device(&self, device: &ObjectPath<'_>) -> Result<()>;

    /// Sets the device discovery filter for the caller. When this method is called with
    /// no filter parameter, filter is removed.
    ///
    /// When discovery filter is set, Device objects will be created as new devices with
    /// matching criteria are discovered regardless of they are connectable or
    /// discoverable which enables listening to non-connectable and non-discoverable
    /// devices.
    ///
    /// When multiple clients call SetDiscoveryFilter, their filters are internally
    /// merged, and notifications about new devices are sent to all clients. Therefore,
    /// each client must check that device updates actually match its filter.
    ///
    /// When SetDiscoveryFilter is called multiple times by the same client, last filter
    /// passed will be active for given client.
    ///
    /// SetDiscoveryFilter can be called before StartDiscovery.
    /// It is useful when client will create first discovery session, to ensure that
    /// proper scan will be started right after call to StartDiscovery.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `NotSupported` - Filter not supported
    /// - `Failed` - Operation failed
    async fn set_discovery_filter(&self, filter: HashMap<String, Value<'_>>) -> Result<()>;

    /// Returns available filters that can be given to SetDiscoveryFilter.
    async fn get_discovery_filters(&self) -> Result<Vec<String>>;

    /// Connects to device without need of performing General Discovery.
    ///
    /// Connection mechanism is similar to Connect method on org.bluez.Device1
    /// interface with exception that this method returns success when physical
    /// connection is established and you can specify bearer to connect with parameter.
    ///
    /// After this method returns, services discovery will continue and any supported
    /// profile will be connected. There is no need for calling Connect on Device1 after
    /// this call. If connection was successful this method returns object path to
    /// created device object or device that already exist.
    ///
    /// [experimental]
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid properties
    /// - `AlreadyExists` - Device already exists
    /// - `NotSupported` - Not supported
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    async fn connect_device(
        &self,
        properties: HashMap<String, Value<'_>>,
    ) -> Result<OwnedObjectPath>;

    /// The Bluetooth device address.
    #[zbus(property)]
    fn address(&self) -> Result<String>;

    /// The Bluetooth Address Type. For dual-mode and BR/EDR only adapter this defaults
    /// to "public". Single mode LE adapters may have either value. With privacy enabled
    /// this contains type of Identity Address and not type of address used for
    /// connection.
    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    /// The Bluetooth system name (pretty hostname).
    ///
    /// This property is either a static system default or controlled by an external
    /// daemon providing access to the pretty hostname configuration.
    #[zbus(property)]
    fn name(&self) -> Result<String>;

    /// The Bluetooth friendly name. This value can be changed.
    ///
    /// In case no alias is set, it will return the system provided name. Setting an
    /// empty string as alias will convert it back to the system provided name.
    ///
    /// When resetting the alias with an empty string, the property will default back to
    /// system name.
    ///
    /// On a well configured system, this property never needs to be changed since it
    /// defaults to the system name and provides the pretty hostname.
    ///
    /// Only if the local name needs to be different from the pretty hostname, this
    /// property should be used as last resort.
    #[zbus(property)]
    fn alias(&self) -> Result<String>;

    /// Sets the Bluetooth friendly name (alias) of the adapter.
    ///
    /// Setting an empty string will revert to the system-provided name.
    #[zbus(property)]
    fn set_alias(&self, alias: &str) -> Result<()>;

    /// The Bluetooth class of device.
    ///
    /// This property represents the value that is either automatically configured by
    /// DMI/ACPI information or provided as static configuration.
    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    /// Set an adapter to connectable or non-connectable. This is a global setting and
    /// should only be used by the settings application.
    ///
    /// Setting this property to false will set the Discoverable property of the adapter
    /// to false as well, which will not be reverted if Connectable is set back to true.
    ///
    /// If required, the application will need to manually set Discoverable to true.
    ///
    /// Note that this property only affects incoming connections.
    #[zbus(property)]
    fn connectable(&self) -> Result<bool>;

    /// Sets whether the adapter is connectable.
    ///
    /// Note: Setting this to false will also set Discoverable to false.
    #[zbus(property)]
    fn set_connectable(&self, connectable: bool) -> Result<()>;

    /// Switch an adapter on or off. This will also set the appropriate connectable
    /// state of the controller.
    ///
    /// The value of this property is not persistent. After restart or unplugging of the
    /// adapter it will reset back to false.
    #[zbus(property)]
    fn powered(&self) -> Result<bool>;

    /// Powers the adapter on or off.
    ///
    /// This will also set the appropriate connectable state of the controller.
    #[zbus(property)]
    fn set_powered(&self, powered: bool) -> Result<()>;

    /// The power state of an adapter.
    ///
    /// The power state will show whether the adapter is turning off, or turning on, as
    /// well as being on or off.
    ///
    /// [experimental]
    #[zbus(property)]
    fn power_state(&self) -> Result<String>;

    /// Switch an adapter to discoverable or non-discoverable to either make it visible
    /// or hide it. This is a global setting and should only be used by the settings
    /// application.
    ///
    /// If the DiscoverableTimeout is set to a non-zero value then the system will set
    /// this value back to false after the timer expired.
    ///
    /// In case the adapter is switched off, setting this value will fail.
    ///
    /// When changing the Powered property the new state of this property will be
    /// updated via a PropertiesChanged signal.
    ///
    /// Default: false
    #[zbus(property)]
    fn discoverable(&self) -> Result<bool>;

    /// Sets whether the adapter is discoverable.
    ///
    /// This is a global setting and should only be used by a settings application.
    #[zbus(property)]
    fn set_discoverable(&self, discoverable: bool) -> Result<()>;

    /// Switch an adapter to pairable or non-pairable. This is a global setting and
    /// should only be used by the settings application.
    ///
    /// Note that this property only affects incoming pairing requests.
    ///
    /// Default: true
    #[zbus(property)]
    fn pairable(&self) -> Result<bool>;

    /// Sets whether the adapter is pairable.
    ///
    /// This is a global setting and should only be used by a settings application.
    #[zbus(property)]
    fn set_pairable(&self, pairable: bool) -> Result<()>;

    /// The pairable timeout in seconds. A value of zero means that the timeout is
    /// disabled and it will stay in pairable mode forever.
    ///
    /// Default: 0
    #[zbus(property)]
    fn pairable_timeout(&self) -> Result<u32>;

    /// Sets the pairable timeout in seconds.
    ///
    /// A value of 0 means that the timeout is disabled and the adapter will stay in pairable mode indefinitely.
    #[zbus(property)]
    fn set_pairable_timeout(&self, timeout: u32) -> Result<()>;

    /// The discoverable timeout in seconds. A value of zero means that the timeout is
    /// disabled and it will stay in discoverable/limited mode forever.
    ///
    /// Default: 180
    #[zbus(property)]
    fn discoverable_timeout(&self) -> Result<u32>;

    /// Sets the discoverable timeout in seconds.
    ///
    /// A value of 0 means that the timeout is disabled and the adapter will stay in discoverable mode indefinitely.
    #[zbus(property)]
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<()>;

    /// Indicates that a device discovery procedure is active.
    #[zbus(property)]
    fn discovering(&self) -> Result<bool>;

    /// List of 128-bit UUIDs that represents the available local services.
    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;

    /// Local Device ID information in modalias format used by the kernel and udev.
    ///
    /// [optional]
    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    /// List of supported roles.
    #[zbus(property)]
    fn roles(&self) -> Result<Vec<String>>;

    /// List of 128-bit UUIDs that represents the experimental features currently
    /// enabled.
    ///
    /// [optional]
    #[zbus(property)]
    fn experimental_features(&self) -> Result<Vec<String>>;

    /// The manufacturer of the device, as a uint16 company identifier defined by the
    /// Core Bluetooth Specification.
    #[zbus(property)]
    fn manufacturer(&self) -> Result<u16>;

    /// The Bluetooth version supported by the device, as a core version code defined by
    /// the Core Bluetooth Specification.
    #[zbus(property)]
    fn version(&self) -> Result<u8>;
}
