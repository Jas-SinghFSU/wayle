use super::{WeatherHeader, helpers};
use crate::{
    i18n::t,
    shell::bar::modules::weather::helpers::{self as weather_helpers, convert_temp_unit},
};

impl WeatherHeader {
    pub(super) fn refresh(&mut self) {
        let Some(weather) = self.weather.weather.get() else {
            return;
        };

        let config_units = self.config.config().modules.weather.units.get();
        let units = convert_temp_unit(config_units);

        self.icon_name =
            weather_helpers::condition_icon(weather.current.condition, weather.current.is_day)
                .to_string();

        self.icon_color_class = weather_helpers::condition_color_class(weather.current.condition);

        self.temp_value = weather_helpers::format_temp_value(weather.current.temperature, units);
        self.temp_unit = weather_helpers::temp_unit_symbol(units);

        self.condition = weather_helpers::condition_label(weather.current.condition);

        self.location = helpers::location_display(
            &weather.location.city,
            weather.location.region.as_deref(),
            &weather.location.country,
        );

        let minutes = helpers::updated_ago_minutes(weather.updated_at);
        self.updated_ago = t!(
            "dropdown-weather-updated-ago",
            minutes = minutes.to_string()
        );
    }

    pub(super) fn refresh_updated_ago(&mut self) {
        let Some(weather) = self.weather.weather.get() else {
            return;
        };

        let minutes = helpers::updated_ago_minutes(weather.updated_at);
        self.updated_ago = t!(
            "dropdown-weather-updated-ago",
            minutes = minutes.to_string()
        );
    }

    pub(super) fn icon_name(&self) -> &str {
        &self.icon_name
    }

    pub(super) fn icon_color_class(&self) -> &str {
        self.icon_color_class
    }

    pub(super) fn temp_value(&self) -> &str {
        &self.temp_value
    }

    pub(super) fn temp_unit(&self) -> &str {
        self.temp_unit
    }

    pub(super) fn condition(&self) -> &str {
        &self.condition
    }

    pub(super) fn location(&self) -> &str {
        &self.location
    }

    pub(super) fn updated_ago(&self) -> &str {
        &self.updated_ago
    }
}
