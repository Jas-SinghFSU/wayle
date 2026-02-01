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
                if let Some(c) = country {
                    format!("{name},{c}")
                } else {
                    name.clone()
                }
            }
        }
    }

    fn parse_location(data: &ApiResponse) -> Location {
        let parts: Vec<&str> = data.resolved_address.split(',').collect();
        let city = parts.first().map_or("", |part| part.trim()).to_string();
        let region = parts.get(1).map(|part| part.trim().to_string());
        let country = parts.last().map_or("", |part| part.trim()).to_string();

        Location {
            city,
            region,
            country,
            lat: data.latitude,
            lon: data.longitude,
        }
    }
}

#[async_trait]
impl WeatherProvider for VisualCrossing {
    fn kind(&self) -> WeatherProviderKind {
        WeatherProviderKind::VisualCrossing
    }

    async fn fetch(&self, location: &LocationQuery) -> Result<Weather> {
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

        Ok(Weather {
            current,
            hourly,
            daily,
            location: Self::parse_location(&data),
            astronomy,
            updated_at: Utc::now(),
        })
    }
}
