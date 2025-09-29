use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_traits::ModelMonitoring;

use super::Device;
use crate::{
    error::Error,
    proxy::device::DeviceProxy,
    types::{BatteryLevel, BatteryTechnology, DeviceState, DeviceType, WarningLevel},
};

impl ModelMonitoring for Device {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = DeviceProxy::new(&self.zbus_connection, self.device_path.clone()).await?;

        let weak_self = Arc::downgrade(&self);
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        monitor_device(weak_self, proxy, cancellation_token.clone()).await
    }
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
async fn monitor_device(
    weak_device: Weak<Device>,
    proxy: DeviceProxy<'static>,
    cancel_token: CancellationToken,
) -> Result<(), Error> {
    let mut native_path_changed = proxy.receive_native_path_changed().await;
    let mut vendor_changed = proxy.receive_vendor_changed().await;
    let mut model_changed = proxy.receive_model_changed().await;
    let mut serial_changed = proxy.receive_serial_changed().await;
    let mut update_time_changed = proxy.receive_update_time_changed().await;
    let mut device_type_changed = proxy.receive_device_type_changed().await;
    let mut power_supply_changed = proxy.receive_power_supply_changed().await;
    let mut has_history_changed = proxy.receive_has_history_changed().await;
    let mut has_statistics_changed = proxy.receive_has_statistics_changed().await;
    let mut online_changed = proxy.receive_online_changed().await;
    let mut energy_changed = proxy.receive_energy_changed().await;
    let mut energy_empty_changed = proxy.receive_energy_empty_changed().await;
    let mut energy_full_changed = proxy.receive_energy_full_changed().await;
    let mut energy_full_design_changed = proxy.receive_energy_full_design_changed().await;
    let mut energy_rate_changed = proxy.receive_energy_rate_changed().await;
    let mut voltage_changed = proxy.receive_voltage_changed().await;
    let mut charge_cycles_changed = proxy.receive_charge_cycles_changed().await;
    let mut luminosity_changed = proxy.receive_luminosity_changed().await;
    let mut time_to_empty_changed = proxy.receive_time_to_empty_changed().await;
    let mut time_to_full_changed = proxy.receive_time_to_full_changed().await;
    let mut percentage_changed = proxy.receive_percentage_changed().await;
    let mut temperature_changed = proxy.receive_temperature_changed().await;
    let mut is_present_changed = proxy.receive_is_present_changed().await;
    let mut state_changed = proxy.receive_state_changed().await;
    let mut is_rechargeable_changed = proxy.receive_is_rechargeable_changed().await;
    let mut capacity_changed = proxy.receive_capacity_changed().await;
    let mut technology_changed = proxy.receive_technology_changed().await;
    let mut warning_level_changed = proxy.receive_warning_level_changed().await;
    let mut battery_level_changed = proxy.receive_battery_level_changed().await;
    let mut icon_name_changed = proxy.receive_icon_name_changed().await;
    let mut charge_start_threshold_changed = proxy.receive_charge_start_threshold_changed().await;
    let mut charge_end_threshold_changed = proxy.receive_charge_end_threshold_changed().await;
    let mut charge_threshold_enabled_changed =
        proxy.receive_charge_threshold_enabled_changed().await;
    let mut charge_threshold_supported_changed =
        proxy.receive_charge_threshold_supported_changed().await;
    let mut charge_threshold_settings_supported_changed = proxy
        .receive_charge_threshold_settings_supported_changed()
        .await;
    let mut voltage_min_design_changed = proxy.receive_voltage_min_design_changed().await;
    let mut voltage_max_design_changed = proxy.receive_voltage_max_design_changed().await;
    let mut capacity_level_changed = proxy.receive_capacity_level_changed().await;

    tokio::spawn(async move {
        loop {
            let Some(device) = weak_device.upgrade() else {
                return;
            };

            tokio::select! {
                    _ = cancel_token.cancelled() => {
                        debug!("Device monitoring cancelled");
                        return;
                    }

                    Some(change) = native_path_changed.next() => {
                        if let Ok(val) = change.get().await {
                            device.native_path.set(val);
                    }
                }
                Some(change) = vendor_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.vendor.set(val);
                    }
                }
                Some(change) = model_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.model.set(val);
                    }
                }
                Some(change) = serial_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.serial.set(val);
                    }
                }
                Some(change) = update_time_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.update_time.set(val);
                    }
                }
                Some(change) = device_type_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.device_type.set(DeviceType::from(val));
                    }
                }
                Some(change) = power_supply_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.power_supply.set(val);
                    }
                }
                Some(change) = has_history_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.has_history.set(val);
                    }
                }
                Some(change) = has_statistics_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.has_statistics.set(val);
                    }
                }
                Some(change) = online_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.online.set(val);
                    }
                }
                Some(change) = energy_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.energy.set(val);
                    }
                }
                Some(change) = energy_empty_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.energy_empty.set(val);
                    }
                }
                Some(change) = energy_full_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.energy_full.set(val);
                    }
                }
                Some(change) = energy_full_design_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.energy_full_design.set(val);
                    }
                }
                Some(change) = energy_rate_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.energy_rate.set(val);
                    }
                }
                Some(change) = voltage_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.voltage.set(val);
                    }
                }
                Some(change) = charge_cycles_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_cycles.set(val);
                    }
                }
                Some(change) = luminosity_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.luminosity.set(val);
                    }
                }
                Some(change) = time_to_empty_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.time_to_empty.set(val);
                    }
                }
                Some(change) = time_to_full_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.time_to_full.set(val);
                    }
                }
                Some(change) = percentage_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.percentage.set(val);
                    }
                }
                Some(change) = temperature_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.temperature.set(val);
                    }
                }
                Some(change) = is_present_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.is_present.set(val);
                    }
                }
                Some(change) = state_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.state.set(DeviceState::from(val));
                    }
                }
                Some(change) = is_rechargeable_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.is_rechargeable.set(val);
                    }
                }
                Some(change) = capacity_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.capacity.set(val);
                    }
                }
                Some(change) = technology_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.technology.set(BatteryTechnology::from(val));
                    }
                }
                Some(change) = warning_level_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.warning_level.set(WarningLevel::from(val));
                    }
                }
                Some(change) = battery_level_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.battery_level.set(BatteryLevel::from(val));
                    }
                }
                Some(change) = icon_name_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.icon_name.set(val);
                    }
                }
                Some(change) = charge_start_threshold_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_start_threshold.set(val);
                    }
                }
                Some(change) = charge_end_threshold_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_end_threshold.set(val);
                    }
                }
                Some(change) = charge_threshold_enabled_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_threshold_enabled.set(val);
                    }
                }
                Some(change) = charge_threshold_supported_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_threshold_supported.set(val);
                    }
                }
                Some(change) = charge_threshold_settings_supported_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.charge_threshold_settings_supported.set(val);
                    }
                }
                Some(change) = voltage_min_design_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.voltage_min_design.set(val);
                    }
                }
                Some(change) = voltage_max_design_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.voltage_max_design.set(val);
                    }
                }
                Some(change) = capacity_level_changed.next() => {
                    if let Ok(val) = change.get().await {
                        device.capacity_level.set(val);
                    }
                }
            }
        }
    });

    Ok(())
}
