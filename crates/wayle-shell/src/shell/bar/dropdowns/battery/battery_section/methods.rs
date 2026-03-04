use relm4::prelude::*;
use tracing::warn;
use wayle_battery::types::DeviceState;

use super::{BatterySection, helpers};
use crate::i18n::t;

impl BatterySection {
    pub(super) fn refresh_battery_state(&mut self) {
        let device = &self.battery.device;

        self.percentage = device.percentage.get();
        self.state = device.state.get();
        self.time_to_empty = device.time_to_empty.get();
        self.time_to_full = device.time_to_full.get();
        self.energy_rate = device.energy_rate.get();
        self.energy = device.energy.get();
        self.energy_full = device.energy_full.get();
        self.capacity = device.capacity.get();
        self.warning_level = device.warning_level.get();
        self.is_present = device.is_present.get();
        self.charge_end_threshold = device.charge_end_threshold.get();
        self.charge_threshold_supported = device.charge_threshold_supported.get();
        self.charge_threshold_enabled = device.charge_threshold_enabled.get();
    }

    pub(super) fn handle_charge_limit_toggled(
        &mut self,
        enabled: bool,
        sender: &ComponentSender<Self>,
    ) {
        self.charge_threshold_enabled = enabled;
        let battery = self.battery.clone();

        sender.oneshot_command(async move {
            if let Err(err) = battery.device.enable_charge_threshold(enabled).await {
                warn!(error = %err, "charge threshold toggle failed");
            }
            super::messages::BatterySectionCmd::BatteryStateChanged
        });
    }

    pub(super) fn is_charging(&self) -> bool {
        matches!(
            self.state,
            DeviceState::Charging | DeviceState::PendingCharge
        )
    }

    pub(super) fn is_fully_charged(&self) -> bool {
        matches!(self.state, DeviceState::FullyCharged)
    }

    pub(super) fn state_label(&self) -> String {
        if helpers::is_low_battery(&self.warning_level) {
            t!("dropdown-battery-critical")
        } else if self.is_charging() {
            t!("dropdown-battery-charging")
        } else if self.is_fully_charged() {
            t!("dropdown-battery-plugged-in")
        } else {
            t!("dropdown-battery-on-battery")
        }
    }

    pub(super) fn time_display(&self) -> String {
        let seconds = if self.is_charging() {
            self.time_to_full
        } else {
            self.time_to_empty
        };

        let Some(parts) = helpers::time_parts(seconds) else {
            return String::new();
        };

        let duration = if parts.hours > 0 {
            t!(
                "dropdown-battery-duration-hm",
                hours = parts.hours.to_string(),
                minutes = format!("{:02}", parts.minutes)
            )
        } else {
            t!(
                "dropdown-battery-duration-m",
                minutes = parts.minutes.to_string()
            )
        };

        if self.is_charging() {
            t!("dropdown-battery-time-until-full", duration = duration)
        } else {
            t!("dropdown-battery-time-remaining", duration = duration)
        }
    }

    pub(super) fn has_time_display(&self) -> bool {
        if self.is_charging() {
            self.time_to_full > 0
        } else {
            self.time_to_empty > 0
        }
    }

    pub(super) fn input_display(&self) -> String {
        t!(
            "dropdown-battery-input-watts",
            watts = helpers::format_watts(self.energy_rate)
        )
    }

    pub(super) fn draw_label(&self) -> String {
        if self.is_charging() {
            t!("dropdown-battery-input")
        } else {
            t!("dropdown-battery-draw")
        }
    }

    pub(super) fn draw_value(&self) -> String {
        helpers::format_watts(self.energy_rate)
    }

    pub(super) fn capacity_label(&self) -> String {
        if self.is_charging() {
            t!("dropdown-battery-charged")
        } else {
            t!("dropdown-battery-capacity")
        }
    }

    pub(super) fn capacity_value(&self) -> String {
        helpers::format_watt_hours(self.energy_full)
    }

    pub(super) fn resume_threshold(&self) -> u32 {
        self.charge_end_threshold.saturating_sub(5)
    }
}
