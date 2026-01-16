/// Bluetooth adapter type definitions
pub mod adapter;
/// Bluetooth agent type definitions
pub mod agent;
/// Bluetooth device type definitions
pub mod device;

pub(crate) const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";
pub(crate) const DEVICE_INTERFACE: &str = "org.bluez.Device1";
pub(crate) const BLUEZ_SERVICE: &str = "org.bluez";

/// Bluetooth UUID represented as a string.
#[allow(clippy::upper_case_acronyms)]
pub type UUID = String;

#[derive(Debug, Clone)]
pub(crate) enum ServiceNotification {
    DeviceConnectionChanged,
}
