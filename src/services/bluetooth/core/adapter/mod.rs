mod controls;
mod monitoring;
use std::sync::Arc;

use controls::AdapterControls;
use monitoring::AdapterMonitor;
use tokio_util::sync::CancellationToken;
use std::collections::HashMap;

use zbus::{Connection, zvariant::{OwnedObjectPath, Value}};

use crate::services::{
    bluetooth::{
        BluetoothError,
        proxy::Adapter1Proxy,
        types::{AdapterRole, AddressType, DiscoveryFilter, PowerState, UUID},
    },
    common::Property,
};
use crate::{unwrap_bool, unwrap_string, unwrap_u8, unwrap_u16, unwrap_u32, unwrap_vec};

/// Bluetooth adapter representation with reactive properties.
#[derive(Debug, Clone)]
pub struct Adapter {
    pub(crate) zbus_connection: Connection,

    /// D-Bus object path for this device.
    pub object_path: OwnedObjectPath,

    /// The Bluetooth device address.
    pub address: Property<String>,

    /// The Bluetooth Address Type. For dual-mode and BR/EDR only adapter this defaults
    /// to "public". Single mode LE adapters may have either value. With privacy enabled
    /// this contains type of Identity Address and not type of address used for
    /// connection.
    pub address_type: Property<AddressType>,

    /// The Bluetooth system name (pretty hostname).
    ///
    /// This property is either a static system default or controlled by an external
    /// daemon providing access to the pretty hostname configuration.
    pub name: Property<String>,

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
    pub alias: Property<String>,

    /// The Bluetooth class of device.
    ///
    /// This property represents the value that is either automatically configured by
    /// DMI/ACPI information or provided as static configuration.
    pub class: Property<u32>,

    /// Set an adapter to connectable or non-connectable. This is a global setting and
    /// should only be used by the settings application.
    ///
    /// Setting this property to false will set the Discoverable property of the adapter
    /// to false as well, which will not be reverted if Connectable is set back to true.
    ///
    /// If required, the application will need to manually set Discoverable to true.
    ///
    /// Note that this property only affects incoming connections.
    pub connectable: Property<bool>,

    /// Switch an adapter on or off. This will also set the appropriate connectable
    /// state of the controller.
    ///
    /// The value of this property is not persistent. After restart or unplugging of the
    /// adapter it will reset back to false.
    pub powered: Property<bool>,

    /// The power state of an adapter.
    ///
    /// The power state will show whether the adapter is turning off, or turning on, as
    /// well as being on or off.
    ///
    /// [experimental]
    pub power_state: Property<PowerState>,

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
    pub discoverable: Property<bool>,

    /// The discoverable timeout in seconds. A value of zero means that the timeout is
    /// disabled and it will stay in discoverable/limited mode forever.
    ///
    /// Default: 180
    pub discoverable_timeout: Property<u32>,

    /// Indicates that a device discovery procedure is active.
    pub discovering: Property<bool>,

    /// Switch an adapter to pairable or non-pairable. This is a global setting and
    /// should only be used by the settings application.
    ///
    /// Note that this property only affects incoming pairing requests.
    ///
    /// Default: true
    pub pairable: Property<bool>,

    /// The pairable timeout in seconds. A value of zero means that the timeout is
    /// disabled and it will stay in pairable mode forever.
    ///
    /// Default: 0
    pub pairable_timeout: Property<u32>,

    /// List of 128-bit UUIDs that represents the available local services.
    pub uuids: Property<Vec<UUID>>,

    /// Local Device ID information in modalias format used by the kernel and udev.
    pub modalias: Property<Option<String>>,

    /// List of supported roles.
    pub roles: Property<Vec<AdapterRole>>,

    /// List of 128-bit UUIDs that represents the experimental features currently
    /// enabled.
    pub experimental_features: Property<Vec<UUID>>,

    /// The manufacturer of the device, as a uint16 company identifier defined by the
    /// Core Bluetooth Specification.
    pub manufacturer: Property<u16>,

    /// The Bluetooth version supported by the device, as a core version code defined by
    /// the Core Bluetooth Specification.
    pub version: Property<u8>,
}

/// Fetched device properties from D-Bus
struct AdapterProperties {
    pub address: String,
    pub address_type: String,
    pub name: String,
    pub alias: String,
    pub class: u32,
    pub connectable: bool,
    pub powered: bool,
    pub power_state: String,
    pub discoverable: bool,
    pub discoverable_timeout: u32,
    pub discovering: bool,
    pub pairable: bool,
    pub pairable_timeout: u32,
    pub uuids: Vec<String>,
    pub modalias: Option<String>,
    pub roles: Vec<String>,
    pub experimental_features: Vec<String>,
    pub manufacturer: u16,
    pub version: u8,
}

impl Adapter {
    pub(crate) async fn get(
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Result<Arc<Self>, BluetoothError> {
        let adapter = Self::from_path(connection, object_path).await?;

        Ok(Arc::new(adapter))
    }

    pub(crate) async fn get_live(
        connection: &Connection,
        object_path: OwnedObjectPath,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, BluetoothError> {
        let adapter = Self::from_path(connection, object_path.clone()).await?;
        let adapter = Arc::new(adapter);

        AdapterMonitor::start(adapter.clone(), connection, object_path, cancellation_token).await?;

        Ok(adapter)
    }

    /// Sets the Bluetooth friendly name (alias) of the adapter.
    ///
    /// Setting an empty string will revert to the system-provided name.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_alias(&self, alias: &str) -> Result<(), BluetoothError> {
        AdapterControls::set_alias(&self.zbus_connection, &self.object_path, alias).await
    }

    /// Sets whether the adapter is connectable.
    ///
    /// Note: Setting this to false will also set Discoverable to false.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_connectable(&self, connectable: bool) -> Result<(), BluetoothError> {
        AdapterControls::set_connectable(&self.zbus_connection, &self.object_path, connectable)
            .await
    }

    /// Powers the adapter on or off.
    ///
    /// This will also set the appropriate connectable state of the controller.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_powered(&self, powered: bool) -> Result<(), BluetoothError> {
        AdapterControls::set_powered(&self.zbus_connection, &self.object_path, powered).await
    }

    /// Sets whether the adapter is discoverable.
    ///
    /// This is a global setting and should only be used by a settings application.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_discoverable(&self, discoverable: bool) -> Result<(), BluetoothError> {
        AdapterControls::set_discoverable(&self.zbus_connection, &self.object_path, discoverable)
            .await
    }

    /// Sets the discoverable timeout in seconds.
    ///
    /// A value of 0 means that the timeout is disabled and the adapter will stay in discoverable mode indefinitely.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), BluetoothError> {
        AdapterControls::set_discoverable_timeout(&self.zbus_connection, &self.object_path, timeout)
            .await
    }

    /// Sets whether the adapter is pairable.
    ///
    /// This is a global setting and should only be used by a settings application.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_pairable(&self, pairable: bool) -> Result<(), BluetoothError> {
        AdapterControls::set_pairable(&self.zbus_connection, &self.object_path, pairable).await
    }

    /// Sets the pairable timeout in seconds.
    ///
    /// A value of 0 means that the timeout is disabled and the adapter will stay in pairable mode indefinitely.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_pairable_timeout(&self, timeout: u32) -> Result<(), BluetoothError> {
        AdapterControls::set_pairable_timeout(&self.zbus_connection, &self.object_path, timeout)
            .await
    }

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
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn set_discovery_filter(
        &self,
        filter: DiscoveryFilter<'_>,
    ) -> Result<(), BluetoothError> {
        AdapterControls::set_discovery_filter(&self.zbus_connection, &self.object_path, filter)
            .await
    }

    /// Starts device discovery session which may include starting an inquiry and/or
    /// scanning procedures and remote device name resolving.
    ///
    /// This process will start creating Device objects as new devices are discovered.
    /// Each client can request a single device discovery session per adapter.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    /// - `InProgress` - Discovery already in progress
    pub async fn start_discovery(&self) -> Result<(), BluetoothError> {
        AdapterControls::start_discovery(&self.zbus_connection, &self.object_path).await
    }

    /// Stops device discovery session started by start_discovery.
    ///
    /// Note that a discovery procedure is shared between all discovery sessions thus
    /// calling stop_discovery will only release a single session and discovery will stop
    /// when all sessions from all clients have finished.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    /// - `NotAuthorized` - Not authorized to stop discovery
    pub async fn stop_discovery(&self) -> Result<(), BluetoothError> {
        AdapterControls::stop_discovery(&self.zbus_connection, &self.object_path).await
    }

    /// Removes the remote device object at the given path including cached information
    /// such as bonding information.
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid device path
    /// - `Failed` - Operation failed
    pub async fn remove_device(&self, device_path: &OwnedObjectPath) -> Result<(), BluetoothError> {
        AdapterControls::remove_device(&self.zbus_connection, &self.object_path, device_path).await
    }

    /// Returns available filters that can be given to set_discovery_filter.
    ///
    /// # Errors
    ///
    /// Returns error if the D-Bus operation fails or the adapter is not available.
    pub async fn get_discovery_filters(&self) -> Result<Vec<String>, BluetoothError> {
        AdapterControls::get_discovery_filters(&self.zbus_connection, &self.object_path).await
    }

    /// Connects to device without need of performing General Discovery.
    ///
    /// Connection mechanism is similar to Device connect method with exception that this
    /// method returns success when physical connection is established and you can specify
    /// bearer to connect with parameter.
    ///
    /// After this method returns, services discovery will continue and any supported
    /// profile will be connected. Returns object path to created device object or device that already exists.
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
    pub async fn connect_device(&self, properties: HashMap<String, Value<'_>>) -> Result<OwnedObjectPath, BluetoothError> {
        AdapterControls::connect_device(&self.zbus_connection, &self.object_path, properties).await
    }

    pub(crate) async fn from_path(
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Result<Self, BluetoothError> {
        let proxy = Adapter1Proxy::new(connection, &object_path).await?;
        let props = Self::fetch_properties(&proxy).await?;
        Ok(Self::from_properties(props, connection, object_path))
    }

    #[allow(clippy::too_many_lines)]
    async fn fetch_properties(
        proxy: &Adapter1Proxy<'_>,
    ) -> Result<AdapterProperties, BluetoothError> {
        let (
            address,
            address_type,
            name,
            alias,
            class,
            connectable,
            powered,
            power_state,
            discoverable,
            discoverable_timeout,
            discovering,
            pairable,
            pairable_timeout,
            uuids,
            modalias,
            roles,
            experimental_features,
            manufacturer,
            version,
        ) = tokio::join!(
            proxy.address(),
            proxy.address_type(),
            proxy.name(),
            proxy.alias(),
            proxy.class(),
            proxy.connectable(),
            proxy.powered(),
            proxy.power_state(),
            proxy.discoverable(),
            proxy.discoverable_timeout(),
            proxy.discovering(),
            proxy.pairable(),
            proxy.pairable_timeout(),
            proxy.uuids(),
            proxy.modalias(),
            proxy.roles(),
            proxy.experimental_features(),
            proxy.manufacturer(),
            proxy.version()
        );

        Ok(AdapterProperties {
            address: unwrap_string!(address),
            address_type: unwrap_string!(address_type),
            name: unwrap_string!(name),
            alias: unwrap_string!(alias),
            class: unwrap_u32!(class),
            connectable: unwrap_bool!(connectable),
            powered: unwrap_bool!(powered),
            power_state: unwrap_string!(power_state),
            discoverable: unwrap_bool!(discoverable),
            discoverable_timeout: unwrap_u32!(discoverable_timeout),
            discovering: unwrap_bool!(discovering),
            pairable: unwrap_bool!(pairable),
            pairable_timeout: unwrap_u32!(pairable_timeout),
            uuids: unwrap_vec!(uuids),
            modalias: modalias.ok(),
            roles: unwrap_vec!(roles),
            experimental_features: unwrap_vec!(experimental_features),
            manufacturer: unwrap_u16!(manufacturer),
            version: unwrap_u8!(version),
        })
    }

    fn from_properties(
        props: AdapterProperties,
        connection: &Connection,
        object_path: OwnedObjectPath,
    ) -> Self {
        Self {
            object_path,
            zbus_connection: connection.clone(),
            address: Property::new(props.address),
            address_type: Property::new(AddressType::from(props.address_type.as_str())),
            name: Property::new(props.name),
            alias: Property::new(props.alias),
            class: Property::new(props.class),
            connectable: Property::new(props.connectable),
            powered: Property::new(props.powered),
            power_state: Property::new(PowerState::from(props.power_state.as_str())),
            discoverable: Property::new(props.discoverable),
            discoverable_timeout: Property::new(props.discoverable_timeout),
            discovering: Property::new(props.discovering),
            pairable: Property::new(props.pairable),
            pairable_timeout: Property::new(props.pairable_timeout),
            uuids: Property::new(
                props
                    .uuids
                    .into_iter()
                    .map(|s| UUID::from(s.as_str()))
                    .collect(),
            ),
            modalias: Property::new(props.modalias),
            roles: Property::new(
                props
                    .roles
                    .into_iter()
                    .map(|s| AdapterRole::from(s.as_str()))
                    .collect(),
            ),
            experimental_features: Property::new(
                props
                    .experimental_features
                    .into_iter()
                    .map(|s| UUID::from(s.as_str()))
                    .collect(),
            ),
            manufacturer: Property::new(props.manufacturer),
            version: Property::new(props.version),
        }
    }
}
