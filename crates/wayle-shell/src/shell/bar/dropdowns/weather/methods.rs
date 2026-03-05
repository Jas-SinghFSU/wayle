use wayle_weather::{LocationQuery, WeatherErrorKind};

use super::WeatherDropdown;
use crate::i18n::t;

impl WeatherDropdown {
    pub(super) fn error_description(&self) -> String {
        match &self.error_kind {
            Some(WeatherErrorKind::ApiKeyMissing { provider }) => {
                t!(
                    "dropdown-weather-error-api-key",
                    provider = provider.as_str()
                )
            }
            Some(WeatherErrorKind::LocationNotFound { query }) => {
                t!("dropdown-weather-error-location", query = query.as_str())
            }
            Some(WeatherErrorKind::Network) => t!("dropdown-weather-error-network"),
            Some(WeatherErrorKind::RateLimited) => t!("dropdown-weather-error-rate-limit"),
            Some(WeatherErrorKind::Other) | None => t!("dropdown-weather-error-unknown"),
        }
    }

    pub(super) fn trigger_refresh(&self) {
        let location = self.config.config().modules.weather.location.get();
        self.weather.set_location(LocationQuery::city(&location));
    }
}
