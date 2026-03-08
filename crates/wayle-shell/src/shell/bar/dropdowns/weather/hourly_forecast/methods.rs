use super::{HourlyForecast, helpers, hourly_item::HourlyItem};
use crate::{
    i18n::t,
    shell::bar::modules::weather::helpers::{self as weather_helpers, convert_temp_unit},
};

const MAX_ITEMS: usize = 5;

impl HourlyForecast {
    pub(super) fn refresh(&mut self) {
        let Some(weather) = self.weather.weather.get() else {
            return;
        };

        let config_units = self.config.config().modules.weather.units.get();
        let units = convert_temp_unit(config_units);

        let mut guard = self.items.guard();
        guard.clear();

        let time_format = self.config.config().modules.weather.time_format.get();

        for (index, hourly) in weather.hourly.iter().take(MAX_ITEMS).enumerate() {
            let time_label = if index == 0 {
                t!("dropdown-weather-now")
            } else {
                helpers::hourly_time_label(hourly.time, time_format)
            };

            guard.push_back(HourlyItem {
                time_label,
                icon_name: weather_helpers::condition_icon(hourly.condition, hourly.is_day)
                    .to_string(),
                icon_color_class: weather_helpers::condition_color_class(hourly.condition),
                temp_value: weather_helpers::format_temp_value(hourly.temperature, units),
            });
        }
    }
}
