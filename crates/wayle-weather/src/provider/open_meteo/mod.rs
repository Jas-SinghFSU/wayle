mod parse;
mod types;

use async_trait::async_trait;
use parse::PROVIDER;
use types::{ApiResponse, ForecastRequest};

use super::{WeatherProvider, build_weather};
use crate::{
    error::{Error, Result},
    model::{Location, LocationQuery, Weather, WeatherProviderKind},
};

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";

const HOURLY_PARAMS: &str = "temperature_2m,relative_humidity_2m,apparent_temperature,\
    precipitation_probability,precipitation,weather_code,cloud_cover,pressure_msl,\
    visibility,wind_speed_10m,wind_direction_10m,wind_gusts_10m,dew_point_2m,uv_index,is_day";

const DAILY_PARAMS: &str = "weather_code,temperature_2m_max,temperature_2m_min,\
    relative_humidity_2m_mean,sunrise,sunset,uv_index_max,precipitation_sum,\
    precipitation_probability_max,wind_speed_10m_max";

/// Open-Meteo weather provider (default, no API key required).
pub struct OpenMeteo {
    client: reqwest::Client,
}

impl OpenMeteo {
    /// Creates a new Open-Meteo provider with a default HTTP client.
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for OpenMeteo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WeatherProvider for OpenMeteo {
    fn kind(&self) -> WeatherProviderKind {
        WeatherProviderKind::OpenMeteo
    }

    async fn fetch(&self, _location: &LocationQuery, resolved: &Location) -> Result<Weather> {
        let request = ForecastRequest {
            latitude: resolved.lat,
            longitude: resolved.lon,
            hourly: HOURLY_PARAMS,
            daily: DAILY_PARAMS,
            temperature_unit: "celsius",
            wind_speed_unit: "kmh",
            timezone: "auto",
            forecast_days: 7,
        };

        let resp = self
            .client
            .get(FORECAST_URL)
            .query(&request)
            .send()
            .await
            .map_err(|err| Error::http(PROVIDER, err))?;

        if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimited { provider: PROVIDER });
        }

        if !resp.status().is_success() {
            return Err(Error::status(PROVIDER, resp.status()));
        }

        let data: ApiResponse = resp
            .json()
            .await
            .map_err(|err| Error::parse(PROVIDER, err.to_string()))?;

        let current = parse::build_current(&data)?;
        let hourly = parse::build_hourly(&data.hourly, 24)?;
        let daily = parse::build_daily(&data, 7)?;

        Ok(build_weather(current, hourly, daily, resolved.clone()))
    }
}
