mod adapter;
mod agent;
mod device;

pub use adapter::*;
pub use agent::*;
pub use device::*;

/// BlueZ D-Bus interface for Bluetooth adapters.
pub const ADAPTER_INTERFACE: &str = "org.bluez.Adapter1";

/// BlueZ D-Bus interface for Bluetooth devices.
pub const DEVICE_INTERFACE: &str = "org.bluez.Device1";

#[allow(clippy::upper_case_acronyms)]
pub type UUID = String;
