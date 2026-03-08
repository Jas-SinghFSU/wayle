use super::{DailyForecast, daily_item::DailyItem, helpers};
use crate::{
    i18n::t,
    shell::bar::modules::weather::helpers::{self as weather_helpers, convert_temp_unit},
};

const MAX_DAYS: usize = 5;
const BAR_WIDTH_REM: f32 = 4.0;
const REM_BASE: f32 = 16.0;

impl DailyForecast {
    pub(super) fn refresh(&mut self) {
        let Some(weather) = self.weather.weather.get() else {
            return;
        };

        let config_units = self.config.config().modules.weather.units.get();
        let units = convert_temp_unit(config_units);

        let temp_pairs: Vec<_> = weather
            .daily
            .iter()
            .take(MAX_DAYS)
            .map(|day| (day.temp_low, day.temp_high))
            .collect();

        let (range_min, range_max) = helpers::temp_range(&temp_pairs);

        let scale = self.config.config().styling.scale.get().value();
        let bar_width_px = (BAR_WIDTH_REM * scale * REM_BASE).round();

        let mut guard = self.items.guard();
        guard.clear();

        for day in weather.daily.iter().take(MAX_DAYS) {
            let is_today = helpers::is_today(day.date);

            let day_label = if is_today {
                t!("dropdown-weather-today")
            } else {
                helpers::day_label(day.date)
            };

            let bar_offsets = helpers::temp_bar_offsets(
                day.temp_low.celsius(),
                day.temp_high.celsius(),
                range_min,
                range_max,
            );

            let bar_margin_start = (bar_offsets.left_pct / 100.0 * bar_width_px) as i32;
            let bar_fill_width = (bar_offsets.width_pct / 100.0 * bar_width_px).max(3.0) as i32;

            guard.push_back(DailyItem {
                day_label,
                icon_name: weather_helpers::condition_icon(day.condition, true).to_string(),
                icon_color_class: weather_helpers::condition_color_class(day.condition),
                condition: weather_helpers::condition_label(day.condition),
                high: weather_helpers::format_temp_value(day.temp_high, units),
                low: weather_helpers::format_temp_value(day.temp_low, units),
                bar_width: bar_width_px as i32,
                bar_margin_start,
                bar_fill_width,
                is_today,
            });
        }
    }
}
