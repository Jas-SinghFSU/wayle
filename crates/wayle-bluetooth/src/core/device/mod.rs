pub(crate) mod controls;
pub(crate) mod monitoring;
pub(crate) mod types;

use std::sync::Arc;

use controls::DeviceControls;
use derive_more::Debug;
use futures::{Stream, StreamExt};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use types::{AdvertisingData, DeviceProperties, DeviceSet, ManufacturerData, ServiceData};
pub use types::{DeviceParams, DisconnectedEvent, LiveDeviceParams};
use wayle_common::{Property, unwrap_bool, unwrap_string};
use wayle_traits::{ModelMonitoring, Reactive};
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    error::Error,
    proxy::{battery::Battery1Proxy, device::Device1Proxy},
    types::{ServiceNotification, UUID, adapter::AddressType, device::PreferredBearer},
};

/// Represents a Bluetooth device with its properties and pairing state.
#[derive(Debug, Clone)]
pub struct Device {
    #[debug(skip)]
    pub(crate) zbus_connection: Connection,
    #[debug(skip)]
    pub(crate) cancellation_token: Option<CancellationToken>,
    #[debug(skip)]
    pub(crate) notifier_tx: broadcast::Sender<ServiceNotification>,

    /// D-Bus object path for this device.
    pub object_path: OwnedObjectPath,

    /// The Bluetooth device address of the remote device.
    pub address: Property<String>,

    /// The Bluetooth device Address Type. For dual-mode and BR/EDR only devices this
    /// defaults to "public". Single mode LE devices may have either value.
    ///
    /// If remote device uses privacy than before pairing this represents address type
    /// used for connection and Identity Address after pairing.
    pub address_type: Property<AddressType>,

    /// The Bluetooth remote name.
    ///
    /// This value is only present for completeness. It is better to always use the
    /// Alias property when displaying the devices name.
    ///
    /// If the Alias property is unset, it will reflect this value which makes it
    /// more convenient.
    pub name: Property<Option<String>>,

    /// Proposed icon name according to the freedesktop.org icon naming specification.
    pub icon: Property<Option<String>>,

    /// Battery charge percentage of the device (0-100).
    ///
    /// Only available for devices that support battery reporting.
    /// `None` if the device doesn't have a battery or doesn't report battery status.
    pub battery_percentage: Property<Option<u8>>,

    /// The Bluetooth class of device of the remote device.
    pub class: Property<Option<u32>>,

    /// External appearance of device, as found on GAP service.
    pub appearance: Property<Option<u16>>,

    /// List of 128-bit UUIDs that represents the available remote services.
    pub uuids: Property<Option<Vec<UUID>>>,

    /// Indicates if the remote device is paired. Paired means the pairing process where
    /// devices exchange the information to establish an encrypted connection has been
    /// completed.
    pub paired: Property<bool>,

    /// Indicate whether or not the device is currently in the process of pairing
    pub pairing: Property<bool>,

    /// Indicates if the remote device is bonded. Bonded means the information exchanged
    /// on pairing process has been stored and will be persisted.
    pub bonded: Property<bool>,

    /// Indicates if the remote device is currently connected.
    ///
    /// A PropertiesChanged signal indicate changes to this status.
    pub connected: Property<bool>,

    /// Indicates if the remote is seen as trusted.
    ///
    /// This setting can be changed by the application.
    pub trusted: Property<bool>,

    /// If set to true any incoming connections from the device will be immediately
    /// rejected.
    ///
    /// Any device drivers will also be removed and no new ones will be probed as long
    /// as the device is blocked.
    pub blocked: Property<bool>,

    /// If set to true this device will be allowed to wake the host from system suspend.
    pub wake_allowed: Property<bool>,

    /// The name alias for the remote device. The alias can be used to have a different
    /// friendly name for the remote device.
    ///
    /// In case no alias is set, it will return the remote device name. Setting an empty
    /// string as alias will convert it back to the remote device name.
    ///
    /// When resetting the alias with an empty string, the property will default back to
    /// the remote name.
    pub alias: Property<String>,

    /// The object path of the adapter the device belongs to.
    pub adapter: Property<OwnedObjectPath>,

    /// Set to true if the device only supports the pre-2.1 pairing mechanism.
    ///
    /// This property is useful during device discovery to anticipate whether legacy or
    /// simple pairing will occur if pairing is initiated.
    ///
    /// Note that this property can exhibit false-positives in the case of Bluetooth 2.1
    /// (or newer) devices that have disabled Extended Inquiry Response support.
    pub legacy_pairing: Property<bool>,

    /// Set to true if the device was cable paired and it doesn't support the canonical
    /// bonding with encryption, e.g. the Sixaxis gamepad.
    ///
    /// If true, BlueZ will establish a connection without enforcing encryption.
    pub cable_pairing: Property<bool>,

    /// Remote Device ID information in modalias format used by the kernel and udev.
    pub modalias: Property<Option<String>>,

    /// Received Signal Strength Indicator of the remote device (inquiry or advertising).
    pub rssi: Property<Option<i16>>,

    /// Advertised transmitted power level (inquiry or advertising).
    pub tx_power: Property<Option<i16>>,

    /// Manufacturer specific advertisement data. Keys are 16 bits Manufacturer ID
    /// followed by its byte array value.
    pub manufacturer_data: Property<Option<ManufacturerData>>,

    /// Service advertisement data. Keys are the UUIDs in string format followed by its
    /// byte array value.
    pub service_data: Property<Option<ServiceData>>,

    /// Indicate whether or not service discovery has been resolved.
    pub services_resolved: Property<bool>,

    /// The Advertising Data Flags of the remote device.
    pub advertising_flags: Property<Vec<u8>>,

    /// The Advertising Data of the remote device. Keys are 1 byte AD Type followed by
    /// data as byte array.
    ///
    /// Note: Only types considered safe to be handled by application are exposed.
    pub advertising_data: Property<AdvertisingData>,

    /// The object paths of the sets the device belongs to followed by a dictionary
    /// which can contain the following:
    ///
    /// - byte Rank: Rank of the device in the Set.
    ///
    /// [experimental]
    pub sets: Property<Vec<DeviceSet>>,

    /// Indicate the preferred bearer when initiating a connection, only available for
    /// dual-mode devices.
    ///
    /// When changing from "bredr" to "le" the device will be removed from the
    /// 'auto-connect' list so it won't automatically be connected when adverting.
    ///
    /// Note: Changes only take effect when the device is disconnected.
    ///
    /// [experimental]
    pub preferred_bearer: Property<Option<PreferredBearer>>,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.object_path == other.object_path
    }
}

impl Reactive for Device {
    type Error = Error;
    type Context<'a> = DeviceParams<'a>;
    type LiveContext<'a> = LiveDeviceParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let device_proxy = Device1Proxy::new(context.connection, &context.path).await?;
        let battery_proxy = Battery1Proxy::new(context.connection, &context.path).await?;
        let props = Self::fetch_properties(&device_proxy, &battery_proxy).await?;
        Ok(Self::from_properties(
            props,
            context.connection,
            context.path,
            context.notifier_tx.clone(),
            None,
        ))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let device_proxy = Device1Proxy::new(context.connection, &context.path).await?;
        let battery_proxy = Battery1Proxy::new(context.connection, &context.path).await?;
        let props = Self::fetch_properties(&device_proxy, &battery_proxy).await?;
        let device = Self::from_properties(
            props,
            context.connection,
            context.path.clone(),
            context.notifier_tx.clone(),
            Some(context.cancellation_token.child_token()),
        );
        let device_arc = Arc::new(device);

        device_arc.clone().start_monitoring().await?;

        Ok(device_arc)
    }
}

impl Device {
    /// Connects all profiles the remote device supports that can be connected to and
    /// have been flagged as auto-connectable. If only subset of profiles is already
    /// connected it will try to connect currently disconnected ones.
    ///
    /// If at least one profile was connected successfully this method will indicate
    /// success.
    ///
    /// For dual-mode devices only one bearer is connected at time, the conditions are
    /// in the following order:
    ///
    /// 1. Connect the disconnected bearer if already connected.
    ///
    /// 2. Connect first the bonded bearer. If no bearers are bonded or both are skip
    ///    and check latest seen bearer.
    ///
    /// 3. Connect last used bearer, in case the timestamps are the same BR/EDR
    ///    takes precedence, or in case PreferredBearer has been set to a specific
    ///    bearer then that is used instead.
    ///
    /// # Errors
    ///
    /// - `NotReady` - Adapter not ready
    /// - `Failed` - Operation failed
    /// - `InProgress` - Connection in progress
    /// - `AlreadyConnected` - Already connected
    /// - `BrEdrProfileUnavailable` - BR/EDR profile unavailable
    pub async fn connect(&self) -> Result<(), Error> {
        DeviceControls::connect(&self.zbus_connection, &self.object_path).await
    }

    /// Disconnects all connected profiles and terminates the low-level ACL connection.
    ///
    /// ACL connection terminates even if some profiles fail to disconnect properly
    /// (e.g., due to misbehaving device).
    ///
    /// Can also cancel a pending Connect call before receiving its reply.
    ///
    /// For non-trusted LE devices, disables incoming connections until Connect is called again.
    ///
    /// # Errors
    ///
    /// - `NotConnected` - Device not connected
    pub async fn disconnect(&self) -> Result<(), Error> {
        DeviceControls::disconnect(&self.zbus_connection, &self.object_path).await
    }

    /// Connects a specific profile of this device. The UUID provided is the remote
    /// service UUID for the profile.
    ///
    /// # Errors
    ///
    /// - `Failed` - Operation failed
    /// - `InProgress` - Connection in progress
    /// - `InvalidArguments` - Invalid UUID
    /// - `NotAvailable` - Profile not available
    /// - `NotReady` - Adapter not ready
    pub async fn connect_profile(&self, profile_uuid: UUID) -> Result<(), Error> {
        DeviceControls::connect_profile(&self.zbus_connection, &self.object_path, profile_uuid)
            .await
    }

    /// Disconnects a specific profile of this device. The profile needs to be
    /// registered client profile.
    ///
    /// There is no connection tracking for a profile, so as long as the profile is
    /// registered this will always succeed.
    ///
    /// # Errors
    ///
    /// - `Failed` - Operation failed
    /// - `InProgress` - Disconnection in progress
    /// - `InvalidArguments` - Invalid UUID
    /// - `NotSupported` - Profile not supported
    pub async fn disconnect_profile(&self, profile_uuid: UUID) -> Result<(), Error> {
        DeviceControls::disconnect_profile(&self.zbus_connection, &self.object_path, profile_uuid)
            .await
    }

    /// Connects to the remote device and initiate pairing procedure then proceed with
    /// service discovery.
    ///
    /// If the application has registered its own agent, then that specific agent will
    /// be used. Otherwise it will use the default agent.
    ///
    /// Only for applications like a pairing wizard it would make sense to have its own
    /// agent. In almost all other cases the default agent will handle this just fine.
    ///
    /// In case there is no application agent and also no default agent present, this
    /// method will fail.
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid arguments
    /// - `Failed` - Operation failed
    /// - `AlreadyExists` - Already paired
    /// - `AuthenticationCanceled` - Authentication canceled
    /// - `AuthenticationFailed` - Authentication failed
    /// - `AuthenticationRejected` - Authentication rejected
    /// - `AuthenticationTimeout` - Authentication timeout
    /// - `ConnectionAttemptFailed` - Connection attempt failed
    pub async fn pair(&self) -> Result<(), Error> {
        DeviceControls::pair(&self.zbus_connection, &self.object_path).await
    }

    /// Cancels a pairing operation initiated by the Pair method.
    ///
    /// # Errors
    ///
    /// - `DoesNotExist` - No pairing in progress
    /// - `Failed` - Operation failed
    pub async fn cancel_pairing(&self) -> Result<(), Error> {
        DeviceControls::cancel_pairing(&self.zbus_connection, &self.object_path).await
    }

    /// Returns all currently known BR/EDR service records for the device. Each
    /// individual byte array represents a raw SDP record, as defined by the Bluetooth
    /// Service Discovery Protocol specification.
    ///
    /// Intended for compatibility layers like Wine that need raw SDP records
    /// for foreign Bluetooth APIs.
    ///
    /// General applications should instead use the Profile API for services-related
    /// functionality.
    ///
    /// [experimental]
    ///
    /// # Errors
    ///
    /// - `Failed` - Operation failed
    /// - `NotReady` - Adapter not ready
    /// - `NotConnected` - Device not connected
    /// - `DoesNotExist` - No service records
    pub async fn get_service_records(&self) -> Result<Vec<Vec<u8>>, Error> {
        DeviceControls::get_service_records(&self.zbus_connection, &self.object_path).await
    }

    /// Sets whether the remote device is trusted.
    ///
    /// Trusted devices can connect without user authorization.
    ///
    /// # Errors
    /// Returns error if D-Bus operation fails or device is not available.
    pub async fn set_trused(&self, trusted: bool) -> Result<(), Error> {
        DeviceControls::set_trusted(&self.zbus_connection, &self.object_path, trusted).await
    }

    /// Sets whether the remote device is blocked.
    ///
    /// Blocked devices will be automatically disconnected and further connections will be denied.
    ///
    /// # Errors
    /// Returns error if D-Bus operation fails or device is not available.
    pub async fn set_blocked(&self, blocked: bool) -> Result<(), Error> {
        DeviceControls::set_blocked(&self.zbus_connection, &self.object_path, blocked).await
    }

    /// Sets whether the device is allowed to wake up the host from system suspend.
    ///
    /// # Errors
    /// Returns error if D-Bus operation fails or device is not available.
    pub async fn set_wake_allowed(&self, wake_allowed: bool) -> Result<(), Error> {
        DeviceControls::set_wake_allowed(&self.zbus_connection, &self.object_path, wake_allowed)
            .await
    }

    /// Sets a custom alias for the remote device.
    ///
    /// Setting an empty string will revert to the remote device's name.
    ///
    /// # Errors
    /// Returns error if D-Bus operation fails or device is not available.
    pub async fn set_alias(&self, alias: &str) -> Result<(), Error> {
        DeviceControls::set_alias(&self.zbus_connection, &self.object_path, alias).await
    }

    /// Sets the preferred bearer for dual-mode devices.
    ///
    /// Possible values: "last-used", "bredr", "le", "last-seen"
    ///
    /// Note: Changes only take effect when the device is disconnected.
    ///
    /// [experimental]
    ///
    /// # Errors
    /// Returns error if D-Bus operation fails or device is not available.
    pub async fn set_preferred_bearer(&self, bearer: &str) -> Result<(), Error> {
        DeviceControls::set_preferred_bearer(&self.zbus_connection, &self.object_path, bearer).await
    }

    /// Removes this device from the adapter and forgets all stored information.
    ///
    /// This will remove the device from the adapter's device list and delete all
    /// pairing/bonding information. The device will need to be rediscovered and
    /// re-paired to connect again.
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid device path
    /// - `DoesNotExist` - Device does not exist
    /// - `Failed` - Operation failed
    pub async fn forget(&self) -> Result<(), Error> {
        DeviceControls::forget(
            &self.zbus_connection,
            &self.adapter.get(),
            &self.object_path,
        )
        .await
    }

    /// This signal is launched when a device is disconnected, with the reason of the
    /// disconnection.
    ///
    /// This could be used by client application, depending on internal policy, to try
    /// to reconnect to the device in case of timeout or unknown disconnection, or to
    /// try to connect to another device.
    ///
    /// # Errors
    /// Returns error if D-Bus proxy creation fails.
    pub async fn disconnected_signal(
        &self,
    ) -> Result<impl Stream<Item = DisconnectedEvent>, Error> {
        let proxy = Device1Proxy::new(&self.zbus_connection, &self.object_path).await?;
        let stream = proxy.receive_disconnected().await?;

        Ok(stream.filter_map(|signal| async move {
            signal.args().ok().map(|args| DisconnectedEvent {
                reason: args.reason,
                message: args.message,
            })
        }))
    }

    #[allow(clippy::too_many_lines)]
    async fn fetch_properties(
        device_proxy: &Device1Proxy<'_>,
        battery_proxy: &Battery1Proxy<'_>,
    ) -> Result<DeviceProperties, Error> {
        let (
            address,
            address_type,
            name,
            icon,
            battery_percentage,
            class,
            appearance,
            uuids,
            paired,
            bonded,
            connected,
            trused,
            blocked,
            wake_allowed,
            alias,
            adapter,
            legacy_pairing,
            cable_pairing,
            modalias,
            rssi,
            tx_power,
            manufacturer_data,
            service_data,
            services_resolved,
            advertising_flags,
            advertising_data,
            sets,
            preferred_bearer,
        ) = tokio::join!(
            device_proxy.address(),
            device_proxy.address_type(),
            device_proxy.name(),
            device_proxy.icon(),
            battery_proxy.percentage(),
            device_proxy.class(),
            device_proxy.appearance(),
            device_proxy.uuids(),
            device_proxy.paired(),
            device_proxy.bonded(),
            device_proxy.connected(),
            device_proxy.trusted(),
            device_proxy.blocked(),
            device_proxy.wake_allowed(),
            device_proxy.alias(),
            device_proxy.adapter(),
            device_proxy.legacy_pairing(),
            device_proxy.cable_pairing(),
            device_proxy.modalias(),
            device_proxy.rssi(),
            device_proxy.tx_power(),
            device_proxy.manufacturer_data(),
            device_proxy.service_data(),
            device_proxy.services_resolved(),
            device_proxy.advertising_flags(),
            device_proxy.advertising_data(),
            device_proxy.sets(),
            device_proxy.preferred_bearer(),
        );

        Ok(DeviceProperties {
            address: unwrap_string!(address),
            address_type: unwrap_string!(address_type),
            name: name.ok(),
            icon: icon.ok(),
            battery_percentage: battery_percentage.ok(),
            class: class.ok(),
            appearance: appearance.ok(),
            uuids: uuids.ok(),
            paired: unwrap_bool!(paired),
            bonded: unwrap_bool!(bonded),
            connected: unwrap_bool!(connected),
            trused: unwrap_bool!(trused),
            blocked: unwrap_bool!(blocked),
            wake_allowed: unwrap_bool!(wake_allowed),
            alias: unwrap_string!(alias),
            adapter: adapter.unwrap_or_default(),
            legacy_pairing: unwrap_bool!(legacy_pairing),
            cable_pairing: unwrap_bool!(cable_pairing),
            modalias: modalias.ok(),
            rssi: rssi.ok(),
            tx_power: tx_power.ok(),
            manufacturer_data: manufacturer_data.ok(),
            service_data: service_data.ok(),
            services_resolved: unwrap_bool!(services_resolved),
            advertising_flags: advertising_flags.unwrap_or_default(),
            advertising_data: advertising_data.unwrap_or_default(),
            sets: sets
                .unwrap_or_default()
                .into_iter()
                .map(|(path, props)| DeviceSet::from_dbus(path, props))
                .collect(),
            preferred_bearer: preferred_bearer.ok(),
        })
    }

    fn from_properties(
        props: DeviceProperties,
        connection: &Connection,
        object_path: OwnedObjectPath,
        notifier_tx: broadcast::Sender<ServiceNotification>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            zbus_connection: connection.clone(),
            cancellation_token,
            notifier_tx,
            object_path,
            address: Property::new(props.address),
            address_type: Property::new(AddressType::from(props.address_type.as_str())),
            name: Property::new(props.name),
            icon: Property::new(props.icon),
            battery_percentage: Property::new(props.battery_percentage),
            class: Property::new(props.class),
            appearance: Property::new(props.appearance),
            uuids: Property::new(props.uuids),
            paired: Property::new(props.paired),
            pairing: Property::new(false),
            bonded: Property::new(props.bonded),
            connected: Property::new(props.connected),
            trusted: Property::new(props.trused),
            blocked: Property::new(props.blocked),
            wake_allowed: Property::new(props.wake_allowed),
            alias: Property::new(props.alias),
            adapter: Property::new(props.adapter),
            legacy_pairing: Property::new(props.legacy_pairing),
            cable_pairing: Property::new(props.cable_pairing),
            modalias: Property::new(props.modalias),
            rssi: Property::new(props.rssi),
            tx_power: Property::new(props.tx_power),
            manufacturer_data: Property::new(props.manufacturer_data),
            service_data: Property::new(props.service_data),
            services_resolved: Property::new(props.services_resolved),
            advertising_flags: Property::new(props.advertising_flags),
            advertising_data: Property::new(props.advertising_data),
            sets: Property::new(props.sets),
            preferred_bearer: Property::new(
                props
                    .preferred_bearer
                    .map(|s| PreferredBearer::from(s.as_str())),
            ),
        }
    }
}
