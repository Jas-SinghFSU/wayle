/// Bluetooth adapter type definitions
pub mod adapter;
/// Bluetooth agent type definitions
pub mod agent;
/// Bluetooth device type definitions
pub mod device;

/// BlueZ D-Bus interface for Bluetooth adapters.
pub(crate) const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";

/// BlueZ D-Bus interface for Bluetooth devices.
pub(crate) const DEVICE_INTERFACE: &str = "org.bluez.Device1";

/// BlueZ D-Bus service path
pub(crate) const BLUEZ_SERVICE: &str = "org.bluez";

/// Bluetooth UUID represented as a string.
#[allow(clippy::upper_case_acronyms)]
pub type UUID = String;

/// Bluetooth service notifications for internal communication.
#[derive(Debug, Clone)]
pub(crate) enum ServiceNotification {
    /// Device connection state changed.
    DeviceConnectionChanged,
}
