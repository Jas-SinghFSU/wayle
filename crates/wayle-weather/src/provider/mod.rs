mod open_meteo;
mod visual_crossing;
mod weatherapi;

use async_trait::async_trait;
use chrono::{NaiveTime, Utc};
pub use open_meteo::OpenMeteo;
pub use visual_crossing::VisualCrossing;
pub use weatherapi::WeatherApi;

use crate::{
    error::{Error, Result},
    model::{
        Astronomy, CurrentWeather, DailyForecast, HourlyForecast, Location, LocationQuery, Weather,
        WeatherProviderKind,
    },
};

/// Trait for weather data providers.
///
/// Each provider implementation fetches weather data from a specific API
/// and normalizes it into the common `Weather` model.
#[async_trait]
pub trait WeatherProvider: Send + Sync {
    /// Returns the provider kind.
    fn kind(&self) -> WeatherProviderKind;

    /// Fetches weather data for the given location.
    ///
    /// # Errors
    ///
    /// Returns error on network failure, invalid location, or API issues.
    async fn fetch(&self, location: &LocationQuery, resolved: &Location) -> Result<Weather>;
}

/// Configuration for creating a weather provider.
pub struct ProviderConfig<'a> {
    /// Which provider to instantiate.
    pub kind: WeatherProviderKind,
    /// API key for Visual Crossing (required if `kind` is `VisualCrossing`).
    pub visual_crossing_key: Option<&'a str>,
    /// API key for WeatherAPI.com (required if `kind` is `WeatherApi`).
    pub weatherapi_key: Option<&'a str>,
}

/// Assembles a `Weather` from parsed provider data.
///
/// Extracts astronomy from the first daily forecast entry, falling back to
/// 06:00 sunrise / 18:00 sunset if daily data is empty.
pub(crate) fn build_weather(
    current: CurrentWeather,
    hourly: Vec<HourlyForecast>,
    daily: Vec<DailyForecast>,
    location: Location,
) -> Weather {
    let astronomy = daily.first().map_or_else(
        || Astronomy {
            sunrise: NaiveTime::from_hms_opt(6, 0, 0).unwrap_or_default(),
            sunset: NaiveTime::from_hms_opt(18, 0, 0).unwrap_or_default(),
        },
        |first_day| Astronomy {
            sunrise: first_day.sunrise,
            sunset: first_day.sunset,
        },
    );

    Weather {
        current,
        hourly,
        daily,
        location,
        astronomy,
        updated_at: Utc::now(),
    }
}

/// Creates a weather provider from configuration.
///
/// # Errors
///
/// Returns `Error::ApiKeyMissing` if the provider requires an API key but none is provided.
pub fn create_provider(config: ProviderConfig<'_>) -> Result<Box<dyn WeatherProvider>> {
    match config.kind {
        WeatherProviderKind::OpenMeteo => Ok(Box::new(OpenMeteo::new())),
        WeatherProviderKind::VisualCrossing => {
            let key = config.visual_crossing_key.ok_or(Error::ApiKeyMissing {
                provider: "visual-crossing",
            })?;
            Ok(Box::new(VisualCrossing::new(key)))
        }
        WeatherProviderKind::WeatherApi => {
            let key = config.weatherapi_key.ok_or(Error::ApiKeyMissing {
                provider: "weatherapi",
            })?;
            Ok(Box::new(WeatherApi::new(key)))
        }
    }
}
