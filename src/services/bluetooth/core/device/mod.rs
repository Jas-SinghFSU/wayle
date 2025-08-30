mod monitoring;

use crate::services::{
    bluetooth::types::{AddressType, PairingRequest, PairingResponder, PreferredBearer, UUID},
    common::Property,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

/// Manufacturer-specific advertisement data keyed by company ID.
pub type ManufacturerData = HashMap<u16, Vec<u8>>;
/// Advertisement data keyed by AD type.
pub type AdvertisingData = HashMap<u8, Vec<u8>>;
/// Service-specific advertisement data keyed by UUID.
pub type ServiceData = HashMap<String, Vec<u8>>;
/// Device set membership with object path and properties.
pub type DeviceSet = (OwnedObjectPath, HashMap<String, OwnedValue>);

/// Represents a Bluetooth device with its properties and pairing state.
#[derive(Debug, Clone)]
pub struct Device {
    pairing_responder: Arc<Mutex<Option<PairingResponder>>>,

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

    /// Active pairing or authorization request awaiting user response.
    ///
    /// Set when BlueZ agent receives a request requiring user interaction.
    /// None when idle or pairing proceeds automatically without user input.
    pub pairing_request: Property<Option<PairingRequest>>,

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
    pub trused: Property<bool>,

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

impl Device {
    pub(crate) fn get() {
        todo!()
    }

    pub(crate) fn get_live() {
        todo!()
    }

    /// Provides a PIN code for legacy device pairing.
    ///
    /// Called in response to `PairingRequest::RequestPinCode`.
    /// PIN must be 1-16 alphanumeric characters.
    ///
    /// # Errors
    ///
    /// Returns error if no PIN request is pending or responder channel is closed.
    pub async fn provide_pin() {
        todo!()
    }

    /// Provides a numeric passkey for device pairing.
    ///
    /// Called in response to `PairingRequest::RequestPasskey`.
    /// Passkey must be between 0-999999.
    ///
    /// # Errors
    ///
    /// Returns error if no passkey request is pending or responder channel is closed.
    pub async fn provide_passkey() {
        todo!()
    }

    /// Provides confirmation for passkey matching.
    ///
    /// Called in response to `PairingRequest::RequestConfirmation`.
    /// Confirms whether displayed passkey matches remote device.
    ///
    /// # Errors
    ///
    /// Returns error if no confirmation request is pending or responder channel is closed.
    pub async fn provide_confirmation() {
        todo!()
    }

    /// Provides authorization for pairing or service connection.
    ///
    /// Called in response to `PairingRequest::RequestAuthorization` or
    /// `PairingRequest::AuthorizeService`.
    ///
    /// # Errors
    ///
    /// Returns error if no authorization request is pending or responder channel is closed.
    pub async fn provide_authorization() {
        todo!()
    }

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
    pub async fn connect() {
        todo!()
    }

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
    pub async fn disconnect() {
        todo!()
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
    pub async fn connect_profile() {
        todo!()
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
    pub async fn disconnect_profile() {
        todo!()
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
    pub async fn pair() {
        todo!()
    }

    /// Cancels a pairing operation initiated by the Pair method.
    ///
    /// # Errors
    ///
    /// - `DoesNotExist` - No pairing in progress
    /// - `Failed` - Operation failed
    pub async fn cancel_pairing() {
        todo!()
    }

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
    pub async fn get_service_records() {
        todo!()
    }

    /// Sets whether the remote device is trusted.
    ///
    /// Trusted devices can connect without user authorization.
    pub fn set_trused() {
        todo!()
    }

    /// Sets whether the remote device is blocked.
    ///
    /// Blocked devices will be automatically disconnected and further connections will be denied.
    pub fn set_blocked() {
        todo!()
    }

    /// Sets whether the device is allowed to wake up the host from system suspend.
    pub fn set_wake_allowed() {
        todo!()
    }

    /// Sets a custom alias for the remote device.
    ///
    /// Setting an empty string will revert to the remote device's name.
    pub fn set_alias() {
        todo!()
    }

    /// Sets the preferred bearer for dual-mode devices.
    ///
    /// Possible values: "last-used", "bredr", "le", "last-seen"
    ///
    /// Note: Changes only take effect when the device is disconnected.
    ///
    /// [experimental]
    pub fn set_preferred_bearer() {
        todo!()
    }
}
