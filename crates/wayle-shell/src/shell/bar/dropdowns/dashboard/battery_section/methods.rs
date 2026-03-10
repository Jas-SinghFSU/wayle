use wayle_battery::types::DeviceState;
use wayle_power_profiles::types::profile::PowerProfile;

use super::BatterySection;
use crate::i18n::t;

const BATTERY_ICON_FULL: f64 = 75.0;
const BATTERY_ICON_MEDIUM: f64 = 50.0;
const BATTERY_ICON_LOW: f64 = 25.0;

pub(super) const BATTERY_WARNING: f64 = 30.0;
pub(super) const BATTERY_CRITICAL: f64 = 15.0;

const SECONDS_PER_HOUR: i64 = 3600;
const SECONDS_PER_MINUTE: i64 = 60;

impl BatterySection {
    pub(super) fn battery_icon(&self) -> &'static str {
        if matches!(self.state, DeviceState::Charging) {
            "ld-battery-charging-symbolic"
        } else if self.percentage > BATTERY_ICON_FULL {
            "ld-battery-full-symbolic"
        } else if self.percentage > BATTERY_ICON_MEDIUM {
            "ld-battery-medium-symbolic"
        } else if self.percentage > BATTERY_ICON_LOW {
            "ld-battery-low-symbolic"
        } else {
            "ld-battery-warning-symbolic"
        }
    }

    pub(super) fn time_remaining_label(&self) -> String {
        if self.time_remaining_secs <= 0 {
            return String::new();
        }

        let hours = self.time_remaining_secs / SECONDS_PER_HOUR;
        let minutes = (self.time_remaining_secs % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;

        if hours > 0 {
            t!(
                "dropdown-dashboard-battery-time-hm",
                hours = hours.to_string(),
                minutes = minutes.to_string()
            )
        } else {
            t!(
                "dropdown-dashboard-battery-time-m",
                minutes = minutes.to_string()
            )
        }
    }

    pub(super) fn power_profile_label(&self) -> String {
        match self.power_profile {
            PowerProfile::PowerSaver => t!("dropdown-dashboard-battery-profile-saver"),
            PowerProfile::Balanced => t!("dropdown-dashboard-battery-profile-balanced"),
            PowerProfile::Performance => t!("dropdown-dashboard-battery-profile-performance"),
        }
    }

    pub(super) fn power_profile_icon(&self) -> &'static str {
        match self.power_profile {
            PowerProfile::PowerSaver => "ld-leaf-symbolic",
            PowerProfile::Balanced => "ld-scale-symbolic",
            PowerProfile::Performance => "ld-zap-symbolic",
        }
    }
}
