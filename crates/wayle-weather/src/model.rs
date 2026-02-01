use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    Distance, Percentage, Precipitation, Pressure, Speed, Temperature, UvIndex, WindDirection,
};

/// Complete weather data from a provider fetch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    /// Real-time weather conditions.
    pub current: CurrentWeather,
    /// Forecasts for upcoming hours.
    pub hourly: Vec<HourlyForecast>,
    /// Forecasts for upcoming days.
    pub daily: Vec<DailyForecast>,
    /// Where this weather data applies.
    pub location: Location,
    /// Sun rise/set times.
    pub astronomy: Astronomy,
    /// When this data was fetched from the provider.
    pub updated_at: DateTime<Utc>,
}

/// Current weather conditions at the location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeather {
    /// Actual air temperature.
    pub temperature: Temperature,
    /// Perceived temperature accounting for wind and humidity.
    pub feels_like: Temperature,
    /// Sky and precipitation state.
    pub condition: WeatherCondition,
    /// Relative humidity percentage.
    pub humidity: Percentage,
    /// Sustained wind speed.
    pub wind_speed: Speed,
    /// Compass direction wind is coming from.
    pub wind_direction: WindDirection,
    /// Peak wind gust speed.
    pub wind_gust: Speed,
    /// Ultraviolet radiation intensity.
    pub uv_index: UvIndex,
    /// Sky obscured by clouds.
    pub cloud_cover: Percentage,
    /// Atmospheric pressure.
    pub pressure: Pressure,
    /// How far you can see.
    pub visibility: Distance,
    /// Temperature at which air becomes saturated.
    pub dewpoint: Temperature,
    /// Rain/snow amount.
    pub precipitation: Precipitation,
    /// True during daylight hours.
    pub is_day: bool,
}

/// Hourly weather forecast entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourlyForecast {
    /// When this forecast applies (UTC).
    pub time: DateTime<Utc>,
    /// Predicted air temperature.
    pub temperature: Temperature,
    /// Predicted apparent temperature.
    pub feels_like: Temperature,
    /// Expected sky/precipitation state.
    pub condition: WeatherCondition,
    /// Expected humidity level.
    pub humidity: Percentage,
    /// Predicted wind speed.
    pub wind_speed: Speed,
    /// Expected wind direction.
    pub wind_direction: WindDirection,
    /// Predicted gust speed.
    pub wind_gust: Speed,
    /// Probability of precipitation.
    pub rain_chance: Percentage,
    /// Expected UV intensity.
    pub uv_index: UvIndex,
    /// Predicted cloud coverage.
    pub cloud_cover: Percentage,
    /// Expected atmospheric pressure.
    pub pressure: Pressure,
    /// Predicted visibility range.
    pub visibility: Distance,
    /// Expected dewpoint.
    pub dewpoint: Temperature,
    /// Predicted precipitation amount.
    pub precipitation: Precipitation,
    /// Whether this hour falls in daylight.
    pub is_day: bool,
}

/// Daily weather forecast entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyForecast {
    /// Calendar date (local timezone).
    pub date: NaiveDate,
    /// Dominant weather condition.
    pub condition: WeatherCondition,
    /// Maximum temperature.
    pub temp_high: Temperature,
    /// Minimum temperature.
    pub temp_low: Temperature,
    /// Mean temperature.
    pub temp_avg: Temperature,
    /// Mean humidity.
    pub humidity_avg: Percentage,
    /// Peak wind speed.
    pub wind_speed_max: Speed,
    /// Chance of rain.
    pub rain_chance: Percentage,
    /// Peak UV index.
    pub uv_index_max: UvIndex,
    /// Total precipitation.
    pub precipitation_sum: Precipitation,
    /// When the sun rises (local time).
    pub sunrise: NaiveTime,
    /// When the sun sets (local time).
    pub sunset: NaiveTime,
}

/// Geographic location for weather data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// City or locality name.
    pub city: String,
    /// State, province, or administrative region.
    pub region: Option<String>,
    /// Country name.
    pub country: String,
    /// Latitude in decimal degrees.
    pub lat: f64,
    /// Longitude in decimal degrees.
    pub lon: f64,
}

/// Astronomical data for the location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Astronomy {
    /// Sunrise time (local).
    pub sunrise: NaiveTime,
    /// Sunset time (local).
    pub sunset: NaiveTime,
}

/// Weather condition categories mapped from provider-specific codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    /// Sunny or clear night sky.
    Clear,
    /// Mix of sun and clouds.
    PartlyCloudy,
    /// Mostly cloudy.
    Cloudy,
    /// Complete cloud cover.
    Overcast,
    /// Light fog or haze.
    Mist,
    /// Dense fog.
    Fog,
    /// Light rain showers.
    LightRain,
    /// Moderate rainfall.
    Rain,
    /// Intense rainfall.
    HeavyRain,
    /// Fine, light rain.
    Drizzle,
    /// Light snowfall.
    LightSnow,
    /// Moderate snowfall.
    Snow,
    /// Intense snowfall.
    HeavySnow,
    /// Mixed rain and snow.
    Sleet,
    /// Lightning and thunder.
    Thunderstorm,
    /// High winds, blustery conditions.
    Windy,
    /// Ice pellets or hail.
    Hail,
    /// Unrecognized condition code.
    Unknown,
}

impl WeatherCondition {
    /// Converts a WMO weather code to a condition category.
    #[must_use]
    pub fn from_wmo_code(code: u8) -> Self {
        match code {
            0 => Self::Clear,
            1..=2 => Self::PartlyCloudy,
            3 => Self::Cloudy,
            44 => Self::Mist,
            45 | 48 => Self::Fog,
            51 | 53 | 55 => Self::Drizzle,
            56 | 57 => Self::Sleet,
            61 => Self::LightRain,
            63 => Self::Rain,
            65 => Self::HeavyRain,
            66 | 67 => Self::Sleet,
            71 => Self::LightSnow,
            73 => Self::Snow,
            75 => Self::HeavySnow,
            77 => Self::Snow,
            80..=82 => Self::Rain,
            85 | 86 => Self::Snow,
            95 => Self::Thunderstorm,
            96 | 99 => Self::Thunderstorm,
            _ => Self::Unknown,
        }
    }

    /// Human-readable name for this condition.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::PartlyCloudy => "Partly Cloudy",
            Self::Cloudy => "Cloudy",
            Self::Overcast => "Overcast",
            Self::Mist => "Mist",
            Self::Fog => "Fog",
            Self::LightRain => "Light Rain",
            Self::Rain => "Rain",
            Self::HeavyRain => "Heavy Rain",
            Self::Drizzle => "Drizzle",
            Self::LightSnow => "Light Snow",
            Self::Snow => "Snow",
            Self::HeavySnow => "Heavy Snow",
            Self::Sleet => "Sleet",
            Self::Thunderstorm => "Thunderstorm",
            Self::Windy => "Windy",
            Self::Hail => "Hail",
            Self::Unknown => "Unknown",
        }
    }
}

impl Display for WeatherCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Query type for weather location lookup.
#[derive(Debug, Clone)]
pub enum LocationQuery {
    /// Lookup by GPS coordinates.
    Coordinates {
        /// Latitude in decimal degrees.
        lat: f64,
        /// Longitude in decimal degrees.
        lon: f64,
    },
    /// Lookup by city name.
    City {
        /// City or locality name.
        name: String,
        /// Optional country to disambiguate.
        country: Option<String>,
    },
}

impl LocationQuery {
    /// Creates a coordinate-based query.
    #[must_use]
    pub fn coords(lat: f64, lon: f64) -> Self {
        Self::Coordinates { lat, lon }
    }

    /// Creates a city name query.
    #[must_use]
    pub fn city(name: impl Into<String>) -> Self {
        Self::City {
            name: name.into(),
            country: None,
        }
    }

    /// Creates a city query with country disambiguation.
    #[must_use]
    pub fn city_country(name: impl Into<String>, country: impl Into<String>) -> Self {
        Self::City {
            name: name.into(),
            country: Some(country.into()),
        }
    }
}

/// Temperature unit for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemperatureUnit {
    /// Celsius.
    #[default]
    Metric,
    /// Fahrenheit.
    Imperial,
}

/// Available weather data providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WeatherProviderKind {
    /// Open-Meteo (free, no API key).
    #[default]
    OpenMeteo,
    /// Visual Crossing (requires API key).
    VisualCrossing,
    /// WeatherAPI.com (requires API key).
    WeatherApi,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wmo_code_0_is_clear() {
        assert_eq!(WeatherCondition::from_wmo_code(0), WeatherCondition::Clear);
    }

    #[test]
    fn wmo_code_1_and_2_are_partly_cloudy() {
        assert_eq!(
            WeatherCondition::from_wmo_code(1),
            WeatherCondition::PartlyCloudy
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(2),
            WeatherCondition::PartlyCloudy
        );
    }

    #[test]
    fn wmo_code_3_is_cloudy() {
        assert_eq!(WeatherCondition::from_wmo_code(3), WeatherCondition::Cloudy);
    }

    #[test]
    fn wmo_code_fog_variants() {
        assert_eq!(WeatherCondition::from_wmo_code(45), WeatherCondition::Fog);
        assert_eq!(WeatherCondition::from_wmo_code(48), WeatherCondition::Fog);
    }

    #[test]
    fn wmo_code_drizzle_variants() {
        assert_eq!(
            WeatherCondition::from_wmo_code(51),
            WeatherCondition::Drizzle
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(53),
            WeatherCondition::Drizzle
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(55),
            WeatherCondition::Drizzle
        );
    }

    #[test]
    fn wmo_code_rain_light_moderate_heavy() {
        assert_eq!(
            WeatherCondition::from_wmo_code(61),
            WeatherCondition::LightRain
        );
        assert_eq!(WeatherCondition::from_wmo_code(63), WeatherCondition::Rain);
        assert_eq!(
            WeatherCondition::from_wmo_code(65),
            WeatherCondition::HeavyRain
        );
    }

    #[test]
    fn wmo_code_snow_light_moderate_heavy() {
        assert_eq!(
            WeatherCondition::from_wmo_code(71),
            WeatherCondition::LightSnow
        );
        assert_eq!(WeatherCondition::from_wmo_code(73), WeatherCondition::Snow);
        assert_eq!(
            WeatherCondition::from_wmo_code(75),
            WeatherCondition::HeavySnow
        );
    }

    #[test]
    fn wmo_code_thunderstorm_variants() {
        assert_eq!(
            WeatherCondition::from_wmo_code(95),
            WeatherCondition::Thunderstorm
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(96),
            WeatherCondition::Thunderstorm
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(99),
            WeatherCondition::Thunderstorm
        );
    }

    #[test]
    fn wmo_code_unknown_for_undefined() {
        assert_eq!(
            WeatherCondition::from_wmo_code(100),
            WeatherCondition::Unknown
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(200),
            WeatherCondition::Unknown
        );
        assert_eq!(
            WeatherCondition::from_wmo_code(255),
            WeatherCondition::Unknown
        );
    }

    #[test]
    fn wmo_code_sleet_variants() {
        assert_eq!(WeatherCondition::from_wmo_code(56), WeatherCondition::Sleet);
        assert_eq!(WeatherCondition::from_wmo_code(57), WeatherCondition::Sleet);
        assert_eq!(WeatherCondition::from_wmo_code(66), WeatherCondition::Sleet);
        assert_eq!(WeatherCondition::from_wmo_code(67), WeatherCondition::Sleet);
    }
}
