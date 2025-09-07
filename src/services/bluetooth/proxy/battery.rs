use zbus::proxy;

/// Proxy for the org.bluez.Battery1 interface.
///
/// Provides battery level information for Bluetooth devices that support battery reporting.
///
/// Service: `org.bluez`
/// Interface: `org.bluez.Battery1`
/// Object path: `[variable prefix]/{hci0,hci1,...}/dev_{BDADDR}`
#[proxy(interface = "org.bluez.Battery1", default_service = "org.bluez")]
pub(crate) trait Battery1 {
    /// The percentage of battery left as an unsigned 8-bit integer.
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<u8>;

    /// Describes where the battery information comes from.
    ///
    /// This property is informational only and may be useful for debugging purposes.
    ///
    /// Providers from org.bluez.BatteryProvider may make use of this property to
    /// indicate where the battery report comes from (e.g. "HFP 1.7", "HID", or the
    /// profile UUID).
    ///
    /// [optional]
    #[zbus(property)]
    fn source(&self) -> zbus::Result<String>;
}
