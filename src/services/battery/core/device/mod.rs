mod controls;
mod monitoring;
/// Type definitions for battery device core functionality
pub mod types;

use std::sync::Arc;

use controls::DeviceController;
use tokio_util::sync::CancellationToken;
use types::{DeviceParams, DeviceProps, LiveDeviceParams};
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    services::{
        battery::{
            Error,
            proxy::device::DeviceProxy,
            types::{BatteryLevel, BatteryTechnology, DeviceState, DeviceType, WarningLevel},
        },
        common::Property,
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_bool, unwrap_f64, unwrap_i32_or, unwrap_i64, unwrap_string, unwrap_u32, unwrap_u64,
};

/// Battery device with reactive properties.
///
/// Provides access to UPower device properties through reactive Property fields
/// that automatically update when the underlying device state changes.
#[derive(Debug, Clone)]
pub struct Device {
    cancellation_token: Option<CancellationToken>,
    zbus_connection: Connection,
    device_path: OwnedObjectPath,

    /// OS specific native path of the power source.
    pub native_path: Property<String>,
    /// Name of the vendor of the battery.
    pub vendor: Property<String>,
    /// Name of the model of this battery.
    pub model: Property<String>,
    /// Unique serial number of the battery.
    pub serial: Property<String>,
    /// The point in time (seconds since the Epoch Jan 1, 1970 0:00 UTC) that data was read from the power source.
    pub update_time: Property<u64>,
    /// Type of power source.
    pub device_type: Property<DeviceType>,
    /// If the power device is used to supply the system.
    pub power_supply: Property<bool>,
    /// If the power device has history.
    pub has_history: Property<bool>,
    /// If the power device has statistics.
    pub has_statistics: Property<bool>,
    /// Whether power is currently being provided through line power.
    pub online: Property<bool>,
    /// Amount of energy (measured in Wh) currently available in the power source.
    pub energy: Property<f64>,
    /// Amount of energy (measured in Wh) in the power source when it's considered to be empty.
    pub energy_empty: Property<f64>,
    /// Amount of energy (measured in Wh) in the power source when it's considered full.
    pub energy_full: Property<f64>,
    /// Amount of energy (measured in Wh) the power source is designed to hold when it's considered full.
    pub energy_full_design: Property<f64>,
    /// Amount of energy being drained from the source, measured in W.
    pub energy_rate: Property<f64>,
    /// Voltage in the Cell or being recorded by the meter.
    pub voltage: Property<f64>,
    /// The number of charge cycles as defined by the TCO certification.
    pub charge_cycles: Property<i32>,
    /// Luminosity being recorded by the meter.
    pub luminosity: Property<f64>,
    /// Number of seconds until the power source is considered empty.
    pub time_to_empty: Property<i64>,
    /// Number of seconds until the power source is considered full.
    pub time_to_full: Property<i64>,
    /// The amount of energy left in the power source expressed as a percentage between 0 and 100.
    pub percentage: Property<f64>,
    /// The temperature of the device in degrees Celsius.
    pub temperature: Property<f64>,
    /// If the power source is present in the bay.
    pub is_present: Property<bool>,
    /// The battery power state.
    pub state: Property<DeviceState>,
    /// If the power source is rechargeable.
    pub is_rechargeable: Property<bool>,
    /// The capacity of the power source expressed as a percentage between 0 and 100.
    pub capacity: Property<f64>,
    /// Technology used in the battery.
    pub technology: Property<BatteryTechnology>,
    /// Warning level of the battery.
    pub warning_level: Property<WarningLevel>,
    /// The level of the battery for devices which do not report a percentage but rather a coarse battery level.
    pub battery_level: Property<BatteryLevel>,
    /// An icon name, following the Icon Naming Specification.
    pub icon_name: Property<String>,
    /// When a start charge threshold is set the battery won't get charged until the charge drops under this threshold.
    pub charge_start_threshold: Property<u32>,
    /// The end charge threshold stops the battery from getting charged after the set threshold.
    pub charge_end_threshold: Property<u32>,
    /// If battery charge start and end limits are applied.
    pub charge_threshold_enabled: Property<bool>,
    /// If setting battery charge limits is supported.
    pub charge_threshold_supported: Property<bool>,
    /// The types of settings for charge thresholds that are supported.
    pub charge_threshold_settings_supported: Property<u32>,
    /// The minimum design voltage of the battery, as reported by the kernel.
    pub voltage_min_design: Property<f64>,
    /// The maximum design voltage of the battery, as reported by the kernel.
    pub voltage_max_design: Property<f64>,
    /// Coarse representation of battery capacity.
    pub capacity_level: Property<String>,
}

impl Reactive for Device {
    type Error = Error;
    type LiveContext<'a> = LiveDeviceParams<'a>;
    type Context<'a> = DeviceParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let device_props = Self::from_connection(context.connection, context.device_path).await?;
        Ok(Self::from_props(
            device_props,
            context.connection,
            context.device_path.clone(),
            None,
        ))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let device_props = Self::from_connection(context.connection, context.device_path).await?;
        let device = Self::from_props(
            device_props,
            context.connection,
            context.device_path.clone(),
            Some(context.cancellation_token.child_token()),
        );

        let device_arc = Arc::new(device);
        device_arc.clone().start_monitoring().await?;

        Ok(device_arc)
    }
}

impl Device {
    /// Refreshes the data collected from the power source.
    ///
    /// # Errors
    /// Returns error if refresh operation fails.
    pub async fn refresh(&self) -> Result<(), Error> {
        DeviceController::refresh(&self.zbus_connection, &self.device_path).await
    }

    /// Gets history for the power device that is persistent across reboots.
    ///
    /// # Arguments
    /// * `history_type` - The type of history. Valid types are "rate" or "charge".
    /// * `timespan` - The amount of data to return in seconds, or 0 for all.
    /// * `resolution` - The approximate number of points to return.
    ///
    /// # Returns
    /// History data as (time, value, state) tuples where:
    /// - time: The time value in seconds from the gettimeofday() method.
    /// - value: The data value, for instance the rate in W or the charge in %.
    /// - state: The state of the device, for instance charging or discharging.
    ///
    /// # Errors
    /// Returns error if getting history fails.
    pub async fn get_history(
        &self,
        history_type: &str,
        timespan: u32,
        resolution: u32,
    ) -> Result<Vec<(u32, f64, u32)>, Error> {
        DeviceController::get_history(
            &self.zbus_connection,
            &self.device_path,
            history_type,
            timespan,
            resolution,
        )
        .await
    }

    /// Gets statistics for the power device that may be interesting to show on a graph.
    ///
    /// # Arguments
    /// * `stat_type` - The mode for the statistics. Valid types are "charging" or "discharging".
    ///
    /// # Returns
    /// Statistics data as (value, accuracy) tuples where:
    /// - value: The value of the percentage point, usually in seconds.
    /// - accuracy: The accuracy of the prediction in percent.
    ///
    /// # Errors
    /// Returns error if getting statistics fails.
    pub async fn get_statistics(&self, stat_type: &str) -> Result<Vec<(f64, f64)>, Error> {
        DeviceController::get_statistics(&self.zbus_connection, &self.device_path, stat_type).await
    }

    /// Limiting the battery charge to the configured thresholds.
    ///
    /// If it is true, the battery charge will be limited to ChargeEndThreshold and start to charge
    /// when the battery is lower than ChargeStartThreshold.
    ///
    /// # Errors
    /// Returns error if setting charge threshold fails.
    pub async fn enable_charge_threshold(&self, enabled: bool) -> Result<(), Error> {
        DeviceController::enable_charge_threshold(&self.zbus_connection, &self.device_path, enabled)
            .await
    }

    #[allow(clippy::too_many_lines)]
    async fn from_connection(
        connection: &Connection,
        device_path: &OwnedObjectPath,
    ) -> Result<DeviceProps, Error> {
        let proxy = DeviceProxy::new(connection, device_path).await?;

        let (
            native_path,
            vendor,
            model,
            serial,
            update_time,
            device_type,
            power_supply,
            has_history,
            has_statistics,
            online,
            energy,
            energy_empty,
            energy_full,
            energy_full_design,
            energy_rate,
            voltage,
            charge_cycles,
            luminosity,
            time_to_empty,
            time_to_full,
            percentage,
            temperature,
            is_present,
            state,
            is_rechargeable,
            capacity,
            technology,
            warning_level,
            battery_level,
            icon_name,
            charge_start_threshold,
            charge_end_threshold,
            charge_threshold_enabled,
            charge_threshold_supported,
            charge_threshold_settings_supported,
            voltage_min_design,
            voltage_max_design,
            capacity_level,
        ) = tokio::join!(
            proxy.native_path(),
            proxy.vendor(),
            proxy.model(),
            proxy.serial(),
            proxy.update_time(),
            proxy.device_type(),
            proxy.power_supply(),
            proxy.has_history(),
            proxy.has_statistics(),
            proxy.online(),
            proxy.energy(),
            proxy.energy_empty(),
            proxy.energy_full(),
            proxy.energy_full_design(),
            proxy.energy_rate(),
            proxy.voltage(),
            proxy.charge_cycles(),
            proxy.luminosity(),
            proxy.time_to_empty(),
            proxy.time_to_full(),
            proxy.percentage(),
            proxy.temperature(),
            proxy.is_present(),
            proxy.state(),
            proxy.is_rechargeable(),
            proxy.capacity(),
            proxy.technology(),
            proxy.warning_level(),
            proxy.battery_level(),
            proxy.icon_name(),
            proxy.charge_start_threshold(),
            proxy.charge_end_threshold(),
            proxy.charge_threshold_enabled(),
            proxy.charge_threshold_supported(),
            proxy.charge_threshold_settings_supported(),
            proxy.voltage_min_design(),
            proxy.voltage_max_design(),
            proxy.capacity_level(),
        );

        Ok(DeviceProps {
            native_path: unwrap_string!(native_path),
            vendor: unwrap_string!(vendor),
            model: unwrap_string!(model),
            serial: unwrap_string!(serial),
            update_time: unwrap_u64!(update_time),
            device_type: unwrap_u32!(device_type),
            power_supply: unwrap_bool!(power_supply),
            has_history: unwrap_bool!(has_history),
            has_statistics: unwrap_bool!(has_statistics),
            online: unwrap_bool!(online),
            energy: unwrap_f64!(energy),
            energy_empty: unwrap_f64!(energy_empty),
            energy_full: unwrap_f64!(energy_full),
            energy_full_design: unwrap_f64!(energy_full_design),
            energy_rate: unwrap_f64!(energy_rate),
            voltage: unwrap_f64!(voltage),
            charge_cycles: unwrap_i32_or!(charge_cycles, -1),
            luminosity: unwrap_f64!(luminosity),
            time_to_empty: unwrap_i64!(time_to_empty),
            time_to_full: unwrap_i64!(time_to_full),
            percentage: unwrap_f64!(percentage),
            temperature: unwrap_f64!(temperature),
            is_present: unwrap_bool!(is_present),
            state: unwrap_u32!(state),
            is_rechargeable: unwrap_bool!(is_rechargeable),
            capacity: unwrap_f64!(capacity),
            technology: unwrap_u32!(technology),
            warning_level: unwrap_u32!(warning_level),
            battery_level: unwrap_u32!(battery_level),
            icon_name: unwrap_string!(icon_name),
            charge_start_threshold: unwrap_u32!(charge_start_threshold),
            charge_end_threshold: unwrap_u32!(charge_end_threshold),
            charge_threshold_enabled: unwrap_bool!(charge_threshold_enabled),
            charge_threshold_supported: unwrap_bool!(charge_threshold_supported),
            charge_threshold_settings_supported: unwrap_u32!(charge_threshold_settings_supported),
            voltage_min_design: unwrap_f64!(voltage_min_design),
            voltage_max_design: unwrap_f64!(voltage_max_design),
            capacity_level: unwrap_string!(capacity_level),
        })
    }

    fn from_props(
        props: DeviceProps,
        connection: &Connection,
        device_path: OwnedObjectPath,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            zbus_connection: connection.clone(),
            device_path,
            cancellation_token,
            native_path: Property::new(props.native_path),
            vendor: Property::new(props.vendor),
            model: Property::new(props.model),
            serial: Property::new(props.serial),
            update_time: Property::new(props.update_time),
            device_type: Property::new(DeviceType::from(props.device_type)),
            power_supply: Property::new(props.power_supply),
            has_history: Property::new(props.has_history),
            has_statistics: Property::new(props.has_statistics),
            online: Property::new(props.online),
            energy: Property::new(props.energy),
            energy_empty: Property::new(props.energy_empty),
            energy_full: Property::new(props.energy_full),
            energy_full_design: Property::new(props.energy_full_design),
            energy_rate: Property::new(props.energy_rate),
            voltage: Property::new(props.voltage),
            charge_cycles: Property::new(props.charge_cycles),
            luminosity: Property::new(props.luminosity),
            time_to_empty: Property::new(props.time_to_empty),
            time_to_full: Property::new(props.time_to_full),
            percentage: Property::new(props.percentage),
            temperature: Property::new(props.temperature),
            is_present: Property::new(props.is_present),
            state: Property::new(DeviceState::from(props.state)),
            is_rechargeable: Property::new(props.is_rechargeable),
            capacity: Property::new(props.capacity),
            technology: Property::new(BatteryTechnology::from(props.technology)),
            warning_level: Property::new(WarningLevel::from(props.warning_level)),
            battery_level: Property::new(BatteryLevel::from(props.battery_level)),
            icon_name: Property::new(props.icon_name),
            charge_start_threshold: Property::new(props.charge_start_threshold),
            charge_end_threshold: Property::new(props.charge_end_threshold),
            charge_threshold_enabled: Property::new(props.charge_threshold_enabled),
            charge_threshold_supported: Property::new(props.charge_threshold_supported),
            charge_threshold_settings_supported: Property::new(
                props.charge_threshold_settings_supported,
            ),
            voltage_min_design: Property::new(props.voltage_min_design),
            voltage_max_design: Property::new(props.voltage_max_design),
            capacity_level: Property::new(props.capacity_level),
        }
    }
}
