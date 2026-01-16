use zbus::proxy;

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower"
)]
pub(crate) trait Device {
    fn refresh(&self) -> zbus::Result<()>;

    fn get_history(
        &self,
        history_type: &str,
        timespan: u32,
        resolution: u32,
    ) -> zbus::Result<Vec<(u32, f64, u32)>>;

    fn get_statistics(&self, stat_type: &str) -> zbus::Result<Vec<(f64, f64)>>;

    fn enable_charge_threshold(&self, charge_threshold: bool) -> zbus::Result<()>;

    #[zbus(property)]
    fn native_path(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn vendor(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn update_time(&self) -> zbus::Result<u64>;

    #[zbus(property, name = "Type")]
    fn device_type(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn power_supply(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_history(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn has_statistics(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn online(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn energy(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_empty(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_full_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn energy_rate(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn voltage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn charge_cycles(&self) -> zbus::Result<i32>;

    #[zbus(property)]
    fn luminosity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn time_to_empty(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn time_to_full(&self) -> zbus::Result<i64>;

    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn temperature(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn is_present(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn is_rechargeable(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn capacity(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn technology(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn warning_level(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn battery_level(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn charge_start_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_end_threshold(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn charge_threshold_enabled(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn charge_threshold_supported(&self) -> zbus::Result<bool>;

    #[zbus(property)]
    fn charge_threshold_settings_supported(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn voltage_min_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn voltage_max_design(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn capacity_level(&self) -> zbus::Result<String>;
}
