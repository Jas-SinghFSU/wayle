mod parse;
mod types;

use async_trait::async_trait;
use chrono::{NaiveTime, Utc};
use parse::PROVIDER;
use types::{ApiResponse, ForecastRequest, GeoLocation, GeocodingRequest, GeocodingResponse};

use crate::{
    error::{Error, Result},
    model::{Astronomy, Location, LocationQuery, Weather, WeatherProviderKind},
    provider::WeatherProvider,
};

const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const GEOCODING_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";

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

    async fn geocode(&self, query: &LocationQuery) -> Result<GeoLocation> {
        match query {
            LocationQuery::Coordinates { lat, lon } => Ok(GeoLocation {
                latitude: *lat,
                longitude: *lon,
                name: String::new(),
                admin1: None,
                country: String::new(),
            }),
            LocationQuery::City { name, country } => {
                let request = GeocodingRequest {
                    name,
                    count: 1,
                    country: country.as_deref(),
                };

                let resp = self
                    .client
                    .get(GEOCODING_URL)
                    .query(&request)
                    .send()
                    .await
                    .map_err(|e| Error::http(PROVIDER, e))?;

                if !resp.status().is_success() {
                    return Err(Error::status(PROVIDER, resp.status()));
                }

                let geo: GeocodingResponse = resp
                    .json()
                    .await
                    .map_err(|e| Error::parse(PROVIDER, e.to_string()))?;

                geo.results
                    .into_iter()
                    .next()
                    .ok_or_else(|| Error::LocationNotFound {
                        query: name.clone(),
                    })
            }
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

    async fn fetch(&self, location: &LocationQuery) -> Result<Weather> {
        let geo = self.geocode(location).await?;

        let request = ForecastRequest {
            latitude: geo.latitude,
            longitude: geo.longitude,
            hourly: HOURLY_PARAMS,
            daily: DAILY_PARAMS,
            temperature_unit: "celsius",
            wind_speed_unit: "kmh",
            timezone: "UTC",
            forecast_days: 7,
        };

        let resp = self
            .client
            .get(FORECAST_URL)
            .query(&request)
            .send()
            .await
            .map_err(|e| Error::http(PROVIDER, e))?;

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
        let hourly = parse::build_hourly(&data.hourly, 24)?;
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
            location: Location {
                city: geo.name,
                region: geo.admin1,
                country: geo.country,
                lat: geo.latitude,
                lon: geo.longitude,
            },
            astronomy,
            updated_at: Utc::now(),
        })
    }
}
