mod parse;
mod types;

use async_trait::async_trait;
use chrono::{NaiveTime, Utc};
use parse::PROVIDER;
use serde::Serialize;
use types::ApiResponse;

use crate::{
    error::{Error, Result},
    model::{Astronomy, Location, LocationQuery, Weather, WeatherProviderKind},
    provider::WeatherProvider,
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
                if let Some(c) = country {
                    format!("{name},{c}")
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

    async fn fetch(&self, location: &LocationQuery) -> Result<Weather> {
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
            .map_err(|e| Error::http(PROVIDER, e))?;

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
            .map_err(|e| Error::parse(PROVIDER, e.to_string()))?;

        let current = parse::build_current(&data)?;
        let hourly = parse::build_hourly(&data, 24)?;
        let daily = parse::build_daily(&data, 7)?;

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

        let loc = &data.location;
        let region = if loc.region.is_empty() {
            None
        } else {
            Some(loc.region.clone())
        };

        Ok(Weather {
            current,
            hourly,
            daily,
            location: Location {
                city: loc.name.clone(),
                region,
                country: loc.country.clone(),
                lat: loc.lat,
                lon: loc.lon,
            },
            astronomy,
            updated_at: Utc::now(),
        })
    }
}
