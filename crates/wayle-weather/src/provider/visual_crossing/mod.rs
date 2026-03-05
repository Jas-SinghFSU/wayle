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

const BASE_URL: &str =
    "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";

#[derive(Serialize)]
struct TimelineRequest<'a> {
    key: &'a str,
    #[serde(rename = "unitGroup")]
    unit_group: &'a str,
    include: &'a str,
    #[serde(rename = "iconSet")]
    icon_set: &'a str,
}

/// Visual Crossing weather provider (requires API key).
pub struct VisualCrossing {
    client: reqwest::Client,
    api_key: String,
}

impl VisualCrossing {
    /// Initializes the provider with the given API key.
    #[must_use]
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    fn location_path(location: &LocationQuery) -> String {
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
impl WeatherProvider for VisualCrossing {
    fn kind(&self) -> WeatherProviderKind {
        WeatherProviderKind::VisualCrossing
    }

    async fn fetch(&self, location: &LocationQuery, resolved: &Location) -> Result<Weather> {
        let location_path = Self::location_path(location);
        let url = format!("{BASE_URL}/{location_path}");

        let request = TimelineRequest {
            key: &self.api_key,
            unit_group: "metric",
            include: "current,hours,days",
            icon_set: "icons2",
        };

        let resp = self
            .client
            .get(&url)
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
