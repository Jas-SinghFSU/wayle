use super::StatsGrid;
use crate::shell::bar::modules::weather::helpers::{self as weather_helpers, convert_temp_unit};

impl StatsGrid {
    pub(super) fn refresh(&mut self) {
        let Some(weather) = self.weather.weather.get() else {
            return;
        };

        let units = convert_temp_unit(self.config.config().modules.weather.units.get());

        self.humidity = format!("{}%", weather.current.humidity.get());
        self.wind = weather_helpers::format_speed(weather.current.wind_speed, units);
        self.uv_index = weather.current.uv_index.get().to_string();
        self.rain_chance = weather
            .daily
            .first()
            .map(|day| format!("{}%", day.rain_chance.get()))
            .unwrap_or_else(|| String::from("--"));
    }

    pub(super) fn humidity(&self) -> &str {
        &self.humidity
    }

    pub(super) fn wind(&self) -> &str {
        &self.wind
    }

    pub(super) fn uv_index(&self) -> &str {
        &self.uv_index
    }

    pub(super) fn rain_chance(&self) -> &str {
        &self.rain_chance
    }
}
