use std::{sync::RwLock, time::Duration};

use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;

use crate::{
    model::{LocationQuery, TemperatureUnit, WeatherProviderKind},
    polling::{self, PollingConfig},
    service::WeatherService,
};

const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(30 * 60);

/// Builder for configuring a [`WeatherService`].
pub struct WeatherServiceBuilder {
    poll_interval: Duration,
    provider_kind: WeatherProviderKind,
    location: LocationQuery,
    units: TemperatureUnit,
    visual_crossing_key: Option<String>,
    weatherapi_key: Option<String>,
}

impl WeatherServiceBuilder {
    /// Creates a new builder with default configuration.
    ///
    /// Defaults to Open-Meteo provider with 30-minute polling interval.
    pub fn new() -> Self {
        Self {
            poll_interval: DEFAULT_POLL_INTERVAL,
            provider_kind: WeatherProviderKind::default(),
            location: LocationQuery::city("San Francisco"),
            units: TemperatureUnit::default(),
            visual_crossing_key: None,
            weatherapi_key: None,
        }
    }

    /// Sets the polling interval for weather updates.
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Sets the weather provider to use.
    pub fn provider(mut self, kind: WeatherProviderKind) -> Self {
        self.provider_kind = kind;
        self
    }

    /// Sets the location for weather data.
    pub fn location(mut self, location: LocationQuery) -> Self {
        self.location = location;
        self
    }

    /// Sets the temperature unit for display.
    pub fn units(mut self, units: TemperatureUnit) -> Self {
        self.units = units;
        self
    }

    /// Sets the Visual Crossing API key.
    pub fn visual_crossing_key(mut self, key: impl Into<String>) -> Self {
        self.visual_crossing_key = Some(key.into());
        self
    }

    /// Sets the WeatherAPI.com API key.
    pub fn weatherapi_key(mut self, key: impl Into<String>) -> Self {
        self.weatherapi_key = Some(key.into());
        self
    }

    /// Builds the service and starts the background polling task.
    ///
    /// If the selected provider requires an API key but none was provided,
    /// the polling loop will log a warning and retry on each interval until
    /// a valid key is set via [`WeatherService::set_visual_crossing_key`] or
    /// [`WeatherService::set_weatherapi_key`].
    #[instrument(skip_all, name = "WeatherService::build")]
    pub fn build(self) -> WeatherService {
        let cancellation_token = CancellationToken::new();
        let weather = Property::new(None);

        let config = PollingConfig {
            kind: self.provider_kind,
            visual_crossing_key: self.visual_crossing_key.clone(),
            weatherapi_key: self.weatherapi_key.clone(),
            location: self.location.clone(),
            poll_interval: self.poll_interval,
        };

        let polling_token = cancellation_token.child_token();
        polling::spawn(polling_token.clone(), weather.clone(), config);

        WeatherService {
            cancellation_token,
            polling_token: RwLock::new(polling_token),
            poll_interval: RwLock::new(self.poll_interval),
            provider_kind: RwLock::new(self.provider_kind),
            location: RwLock::new(self.location),
            units: RwLock::new(self.units),
            visual_crossing_key: RwLock::new(self.visual_crossing_key),
            weatherapi_key: RwLock::new(self.weatherapi_key),
            weather,
        }
    }
}

impl Default for WeatherServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
