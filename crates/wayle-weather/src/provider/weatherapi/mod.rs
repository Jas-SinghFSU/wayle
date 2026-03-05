mod parse;
mod types;

use async_trait::async_trait;
use parse::PROVIDER;
use serde::Serialize;
use types::ApiResponse;

use super::{WeatherProvider, build_weather};
use crate::{
    error::{Error, Result},
    model::{Location, LocationQuery, Weather, WeatherProviderKind},
};

const BASE_URL: &str = "https://api.weatherapi.com/v1/forecast.json";

#[derive(Serialize)]
struct ForecastRequest<'a> {
    key: &'a str,
    q: String,
    days: i32,
    aqi: &'a str,
    alerts: &'a str,
}

/// WeatherAPI.com provider (requires API key).
pub struct WeatherApi {
    client: reqwest::Client,
    api_key: String,
}

impl WeatherApi {
    /// Constructs a provider configured with the specified API key.
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    fn location_query(location: &LocationQuery) -> String {
        match location {
            LocationQuery::Coordinates { lat, lon } => format!("{lat},{lon}"),
            LocationQuery::City { name, country } => {
                if let Some(country_code) = country {
                    format!("{name},{country_code}")
                } else {
                    name.clone()
                }
            }
        }
    }
}

#[async_trait]
impl WeatherProvider for WeatherApi {
    fn kind(&self) -> WeatherProviderKind {
        WeatherProviderKind::WeatherApi
    }

    async fn fetch(&self, location: &LocationQuery, resolved: &Location) -> Result<Weather> {
        let request = ForecastRequest {
            key: &self.api_key,
            q: Self::location_query(location),
            days: 7,
            aqi: "no",
            alerts: "no",
        };

        let resp = self
            .client
            .get(BASE_URL)
            .query(&request)
            .send()
            .await
            .map_err(|err| Error::http(PROVIDER, err))?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED
            || resp.status() == reqwest::StatusCode::FORBIDDEN
        {
            return Err(Error::ApiKeyMissing { provider: PROVIDER });
        }

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
        let hourly = parse::build_hourly(&data, 24)?;
        let daily = parse::build_daily(&data, 7)?;

        Ok(build_weather(current, hourly, daily, resolved.clone()))
    }
}
