#![allow(missing_docs)]
use std::collections::HashMap;

use zbus::{
    Result, proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

/// Proxy for the org.bluez.Device1 interface.
///
/// BlueZ D-Bus Device API for managing Bluetooth devices.
///
/// Service: `org.bluez`
/// Interface: `org.bluez.Device1`
/// Object path: `[variable prefix]/{hci0,hci1,...}/dev_{BDADDR}`
#[proxy(interface = "org.bluez.Device1", default_service = "org.bluez")]
pub(crate) trait Device1 {
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
    async fn connect(&self) -> Result<()>;

    /// Disconnects all connected profiles and then terminates low-level ACL connection.
    ///
    /// ACL connection will be terminated even if some profiles were not disconnected
    /// properly e.g. due to misbehaving device.
    ///
    /// This method can be also used to cancel a preceding Connect call before a reply
    /// to it has been received.
    ///
    /// For non-trusted devices connected over LE bearer calling this method will
    /// disable incoming connections until Connect method is called again.
    ///
    /// # Errors
    ///
    /// - `NotConnected` - Device not connected
    async fn disconnect(&self) -> Result<()>;

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
    async fn connect_profile(&self, uuid: &str) -> Result<()>;

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
    async fn disconnect_profile(&self, uuid: &str) -> Result<()>;

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
    async fn pair(&self) -> Result<()>;

    /// Cancels a pairing operation initiated by the Pair method.
    ///
    /// # Errors
    ///
    /// - `DoesNotExist` - No pairing in progress
    /// - `Failed` - Operation failed
    async fn cancel_pairing(&self) -> Result<()>;

    /// Returns all currently known BR/EDR service records for the device. Each
    /// individual byte array represents a raw SDP record, as defined by the Bluetooth
    /// Service Discovery Protocol specification.
    ///
    /// This method is intended to be only used by compatibility layers like Wine, that
    /// need to provide access to raw SDP records to support foreign Bluetooth APIs.
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
    async fn get_service_records(&self) -> Result<Vec<Vec<u8>>>;

    /// The Bluetooth device address of the remote device.
    #[zbus(property)]
    fn address(&self) -> Result<String>;

    /// The Bluetooth device Address Type. For dual-mode and BR/EDR only devices this
    /// defaults to "public". Single mode LE devices may have either value.
    ///
    /// If remote device uses privacy than before pairing this represents address type
    /// used for connection and Identity Address after pairing.
    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    /// The Bluetooth remote name.
    ///
    /// This value is only present for completeness. It is better to always use the
    /// Alias property when displaying the devices name.
    ///
    /// If the Alias property is unset, it will reflect this value which makes it
    /// more convenient.
    ///
    /// [optional]
    #[zbus(property)]
    fn name(&self) -> Result<String>;

    /// Proposed icon name according to the freedesktop.org icon naming specification.
    ///
    /// [optional]
    #[zbus(property)]
    fn icon(&self) -> Result<String>;

    /// The Bluetooth class of device of the remote device.
    ///
    /// [optional]
    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    /// External appearance of device, as found on GAP service.
    ///
    /// [optional]
    #[zbus(property)]
    fn appearance(&self) -> Result<u16>;

    /// List of 128-bit UUIDs that represents the available remote services.
    ///
    /// [optional]
    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;

    /// Indicates if the remote device is paired. Paired means the pairing process where
    /// devices exchange the information to establish an encrypted connection has been
    /// completed.
    #[zbus(property)]
    fn paired(&self) -> Result<bool>;

    /// Indicates if the remote device is bonded. Bonded means the information exchanged
    /// on pairing process has been stored and will be persisted.
    #[zbus(property)]
    fn bonded(&self) -> Result<bool>;

    /// Indicates if the remote device is currently connected.
    ///
    /// A PropertiesChanged signal indicate changes to this status.
    #[zbus(property)]
    fn connected(&self) -> Result<bool>;

    /// Indicates if the remote is seen as trusted.
    ///
    /// This setting can be changed by the application.
    #[zbus(property)]
    fn trusted(&self) -> Result<bool>;

    /// Sets whether the remote device is trusted.
    ///
    /// Trusted devices can connect without user authorization.
    #[zbus(property)]
    fn set_trusted(&self, trusted: bool) -> Result<()>;

    /// If set to true any incoming connections from the device will be immediately
    /// rejected.
    ///
    /// Any device drivers will also be removed and no new ones will be probed as long
    /// as the device is blocked.
    #[zbus(property)]
    fn blocked(&self) -> Result<bool>;

    /// Sets whether the remote device is blocked.
    ///
    /// Blocked devices will be automatically disconnected and further connections will be denied.
    #[zbus(property)]
    fn set_blocked(&self, blocked: bool) -> Result<()>;

    /// If set to true this device will be allowed to wake the host from system suspend.
    #[zbus(property)]
    fn wake_allowed(&self) -> Result<bool>;

    /// Sets whether the device is allowed to wake up the host from system suspend.
    #[zbus(property)]
    fn set_wake_allowed(&self, allowed: bool) -> Result<()>;

    /// The name alias for the remote device. The alias can be used to have a different
    /// friendly name for the remote device.
    ///
    /// In case no alias is set, it will return the remote device name. Setting an empty
    /// string as alias will convert it back to the remote device name.
    ///
    /// When resetting the alias with an empty string, the property will default back to
    /// the remote name.
    #[zbus(property)]
    fn alias(&self) -> Result<String>;

    /// Sets a custom alias for the remote device.
    ///
    /// Setting an empty string will revert to the remote device's name.
    #[zbus(property)]
    fn set_alias(&self, alias: &str) -> Result<()>;

    /// The object path of the adapter the device belongs to.
    #[zbus(property)]
    fn adapter(&self) -> Result<OwnedObjectPath>;

    /// Set to true if the device only supports the pre-2.1 pairing mechanism.
    ///
    /// This property is useful during device discovery to anticipate whether legacy or
    /// simple pairing will occur if pairing is initiated.
    ///
    /// Note that this property can exhibit false-positives in the case of Bluetooth 2.1
    /// (or newer) devices that have disabled Extended Inquiry Response support.
    #[zbus(property)]
    fn legacy_pairing(&self) -> Result<bool>;

    /// Set to true if the device was cable paired and it doesn't support the canonical
    /// bonding with encryption, e.g. the Sixaxis gamepad.
    ///
    /// If true, BlueZ will establish a connection without enforcing encryption.
    #[zbus(property)]
    fn cable_pairing(&self) -> Result<bool>;

    /// Remote Device ID information in modalias format used by the kernel and udev.
    ///
    /// [optional]
    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    /// Received Signal Strength Indicator of the remote device (inquiry or
    /// advertising).
    ///
    /// [optional]
    #[zbus(property)]
    fn rssi(&self) -> Result<i16>;

    /// Advertised transmitted power level (inquiry or advertising).
    ///
    /// [optional]
    #[zbus(property)]
    fn tx_power(&self) -> Result<i16>;

    /// Manufacturer specific advertisement data. Keys are 16 bits Manufacturer ID
    /// followed by its byte array value.
    ///
    /// [optional]
    #[zbus(property)]
    fn manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>>;

    /// Service advertisement data. Keys are the UUIDs in string format followed by its
    /// byte array value.
    ///
    /// [optional]
    #[zbus(property)]
    fn service_data(&self) -> Result<HashMap<String, Vec<u8>>>;

    /// Indicate whether or not service discovery has been resolved.
    #[zbus(property)]
    fn services_resolved(&self) -> Result<bool>;

    /// The Advertising Data Flags of the remote device.
    #[zbus(property)]
    fn advertising_flags(&self) -> Result<Vec<u8>>;

    /// The Advertising Data of the remote device. Keys are 1 byte AD Type followed by
    /// data as byte array.
    ///
    /// Note: Only types considered safe to be handled by application are exposed.
    #[zbus(property)]
    fn advertising_data(&self) -> Result<HashMap<u8, Vec<u8>>>;

    /// The object paths of the sets the device belongs to followed by a dictionary
    /// which can contain the following:
    ///
    /// - byte Rank: Rank of the device in the Set.
    ///
    /// [experimental]
    #[zbus(property)]
    fn sets(&self) -> Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;

    /// Indicate the preferred bearer when initiating a connection, only available for
    /// dual-mode devices.
    ///
    /// When changing from "bredr" to "le" the device will be removed from the
    /// 'auto-connect' list so it won't automatically be connected when adverting.
    ///
    /// Note: Changes only take effect when the device is disconnected.
    ///
    /// [optional, experimental]
    #[zbus(property)]
    fn preferred_bearer(&self) -> Result<String>;

    /// Sets the preferred bearer for dual-mode devices.
    ///
    /// Possible values: "last-used", "bredr", "le", "last-seen"
    ///
    /// Note: Changes only take effect when the device is disconnected.
    ///
    /// [experimental]
    #[zbus(property)]
    fn set_preferred_bearer(&self, bearer: &str) -> Result<()>;

    /// This signal is launched when a device is disconnected, with the reason of the
    /// disconnection.
    ///
    /// This could be used by client application, depending on internal policy, to try
    /// to reconnect to the device in case of timeout or unknown disconnection, or to
    /// try to connect to another device.
    #[zbus(signal)]
    async fn disconnected(&self, reason: u8, message: String) -> Result<()>;
}
