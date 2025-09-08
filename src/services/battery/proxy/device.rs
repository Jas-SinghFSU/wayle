use zbus::proxy;

/// Proxy for the org.freedesktop.UPower.Device interface
#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower"
)]
pub(crate) trait Device {
    /// Refreshes the data collected from the power source.
    fn refresh(&self) -> zbus::Result<()>;

    /// Gets history for the power device that is persistent across reboots.
    fn get_history(
        &self,
        history_type: &str,
        timespan: u32,
        resolution: u32,
    ) -> zbus::Result<Vec<(u32, f64, u32)>>;

    /// Gets statistics for the power device that may be interesting to show on a graph.
    fn get_statistics(&self, stat_type: &str) -> zbus::Result<Vec<(f64, f64)>>;

    /// Limiting the battery charge to the configured thresholds.
    fn enable_charge_threshold(&self, charge_threshold: bool) -> zbus::Result<()>;

    /// OS specific native path of the power source.
    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    /// Name of the vendor of the battery.
    #[zbus(property)]
    fn vendor(&self) -> zbus::Result<String>;

    /// Name of the model of this battery.
    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    /// Unique serial number of the battery.
    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    /// The point in time that data was read from the power source.
    #[zbus(property)]
    fn update_time(&self) -> zbus::Result<u64>;

    /// Type of power source.
    #[zbus(property, name = "Type")]
    fn device_type(&self) -> zbus::Result<u32>;

    /// If the power device is used to supply the system.
    #[zbus(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    /// If the power device has history.
    #[zbus(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    /// If the power device has statistics.
    #[zbus(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    /// Whether power is currently being provided through line power.
    #[zbus(property)]
    fn online(&self) -> zbus::Result<bool>;

    /// Amount of energy (measured in Wh) currently available in the power source.
    #[zbus(property)]
    fn energy(&self) -> zbus::Result<f64>;

    /// Amount of energy (measured in Wh) in the power source when it's considered to be empty.
    #[zbus(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    /// Amount of energy (measured in Wh) in the power source when it's considered full.
    #[zbus(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    /// Amount of energy (measured in Wh) the power source is designed to hold when it's considered full.
    #[zbus(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    /// Amount of energy being drained from the source, measured in W.
    #[zbus(property)]
    fn energy_rate(&self) -> zbus::Result<f64>;

    /// Voltage in the Cell or being recorded by the meter.
    #[zbus(property)]
    fn voltage(&self) -> zbus::Result<f64>;

    /// The number of charge cycles.
    #[zbus(property)]
    fn charge_cycles(&self) -> zbus::Result<i32>;

    /// Luminosity being recorded by the meter.
    #[zbus(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    /// Number of seconds until the power source is considered empty.
    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    /// Number of seconds until the power source is considered full.
    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    /// The amount of energy left in the power source expressed as a percentage.
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    /// The temperature of the device in degrees Celsius.
    #[zbus(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    /// If the power source is present in the bay.
    #[zbus(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    /// The battery power state.
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// If the power source is rechargeable.
    #[zbus(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    /// The capacity of the power source expressed as a percentage.
    #[zbus(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    /// Technology used in the battery.
    #[zbus(property)]
    fn technology(&self) -> zbus::Result<u32>;

    /// Warning level of the battery.
    #[zbus(property)]
    fn warning_level(&self) -> zbus::Result<u32>;

    /// The level of the battery for devices which do not report a percentage.
    #[zbus(property)]
    fn battery_level(&self) -> zbus::Result<u32>;

    /// An icon name, following the Icon Naming Specification.
    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    /// Battery charge start threshold.
    #[zbus(property)]
    fn charge_start_threshold(&self) -> zbus::Result<u32>;

    /// Battery charge end threshold.
    #[zbus(property)]
    fn charge_end_threshold(&self) -> zbus::Result<u32>;

    /// If battery charge start and end limits are applied.
    #[zbus(property)]
    fn charge_threshold_enabled(&self) -> zbus::Result<bool>;

    /// If setting battery charge limits is supported.
    #[zbus(property)]
    fn charge_threshold_supported(&self) -> zbus::Result<bool>;

    /// The types of settings for charge thresholds that are supported.
    #[zbus(property)]
    fn charge_threshold_settings_supported(&self) -> zbus::Result<u32>;

    /// The minimum design voltage of the battery.
    #[zbus(property)]
    fn voltage_min_design(&self) -> zbus::Result<f64>;

    /// The maximum design voltage of the battery.
    #[zbus(property)]
    fn voltage_max_design(&self) -> zbus::Result<f64>;

    /// Coarse representation of battery capacity.
    #[zbus(property)]
    fn capacity_level(&self) -> zbus::Result<String>;
}
