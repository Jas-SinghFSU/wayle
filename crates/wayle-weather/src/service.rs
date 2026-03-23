use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_core::Property;

use crate::{
    builder::WeatherServiceBuilder,
    error::Error,
    model::{LocationQuery, TemperatureUnit, Weather, WeatherProviderKind},
    polling::{self, PollingConfig},
};

/// Categorized error for UI display without implementation details.
#[derive(Debug, Clone, PartialEq)]
pub enum WeatherErrorKind {
    /// Provider requires an API key that isn't configured.
    ApiKeyMissing {
        /// Display name of the provider (e.g. "Visual Crossing").
        provider: String,
    },
    /// Geocoding could not resolve the requested location.
    LocationNotFound {
        /// The search query that failed to resolve.
        query: String,
    },
    /// Network-level failure (DNS, timeout, connection refused).
    Network,
    /// Provider rate limit exceeded.
    RateLimited,
    /// Anything else (parse errors, unexpected status codes, etc.).
    Other,
}

impl From<&Error> for WeatherErrorKind {
    fn from(err: &Error) -> Self {
        match err {
            Error::ApiKeyMissing { provider } => Self::ApiKeyMissing {
                provider: (*provider).to_owned(),
            },
            Error::LocationNotFound { query } => Self::LocationNotFound {
                query: query.clone(),
            },
            Error::Http { .. } => Self::Network,
            Error::RateLimited { .. } => Self::RateLimited,
            Error::ProviderStatus { .. }
            | Error::Parse { .. }
            | Error::InvalidLocation { .. }
            | Error::NotAvailable => Self::Other,
        }
    }
}

/// Fetch lifecycle state exposed to UI consumers.
#[derive(Debug, Clone, PartialEq)]
pub enum WeatherStatus {
    /// Initial state or refresh in progress.
    Loading,
    /// At least one successful fetch has completed.
    Loaded,
    /// All retries exhausted or non-retryable error.
    Error(WeatherErrorKind),
}

/// Weather service for fetching and caching weather data.
///
/// Provides reactive access to weather data through `Property<Option<Arc<Weather>>>`.
/// The service polls weather data at configurable intervals and caches the
/// last successful fetch.
///
/// All configuration can be changed at runtime via setter methods:
/// - [`set_poll_interval`](Self::set_poll_interval) - Change polling frequency
/// - [`set_location`](Self::set_location) - Change weather location
/// - [`set_units`](Self::set_units) - Change temperature units
/// - [`set_provider`](Self::set_provider) - Change weather provider
#[derive(Debug)]
pub struct WeatherService {
    pub(crate) cancellation_token: CancellationToken,
    pub(crate) polling_token: RwLock<CancellationToken>,
    pub(crate) poll_interval: RwLock<Duration>,
    pub(crate) provider_kind: RwLock<WeatherProviderKind>,
    pub(crate) location: RwLock<LocationQuery>,
    pub(crate) units: RwLock<TemperatureUnit>,
    pub(crate) visual_crossing_key: RwLock<Option<String>>,
    pub(crate) weatherapi_key: RwLock<Option<String>>,

    /// Current weather data. `None` until first successful fetch.
    pub weather: Property<Option<Arc<Weather>>>,

    /// Current fetch lifecycle state.
    pub status: Property<WeatherStatus>,
}

impl WeatherService {
    /// Returns a builder for configuring the weather service.
    pub fn builder() -> WeatherServiceBuilder {
        WeatherServiceBuilder::new()
    }

    /// Updates the polling interval.
    pub fn set_poll_interval(&self, interval: Duration) {
        debug!(?interval, "Updating weather polling interval");
        if let Ok(mut guard) = self.poll_interval.write() {
            *guard = interval;
        }
        self.restart_polling();
    }

    /// Updates the weather location.
    pub fn set_location(&self, location: LocationQuery) {
        debug!(?location, "Updating weather location");
        if let Ok(mut guard) = self.location.write() {
            *guard = location;
        }
        self.restart_polling();
    }

    /// Updates the temperature units.
    pub fn set_units(&self, units: TemperatureUnit) {
        debug!(?units, "Updating temperature units");
        if let Ok(mut guard) = self.units.write() {
            *guard = units;
        }
        self.restart_polling();
    }

    /// Updates the weather provider.
    ///
    /// If the provider requires an API key that wasn't set, the polling loop
    /// will log a warning and retry on the next interval.
    pub fn set_provider(&self, kind: WeatherProviderKind) {
        debug!(?kind, "Updating weather provider");
        if let Ok(mut guard) = self.provider_kind.write() {
            *guard = kind;
        }
        self.restart_polling();
    }

    /// Updates the Visual Crossing API key.
    pub fn set_visual_crossing_key(&self, key: Option<String>) {
        debug!("Updating Visual Crossing API key");
        if let Ok(mut guard) = self.visual_crossing_key.write() {
            *guard = key;
        }
        self.restart_polling();
    }

    /// Updates the WeatherAPI.com API key.
    pub fn set_weatherapi_key(&self, key: Option<String>) {
        debug!("Updating WeatherAPI.com API key");
        if let Ok(mut guard) = self.weatherapi_key.write() {
            *guard = key;
        }
        self.restart_polling();
    }

    fn restart_polling(&self) {
        self.status.set(WeatherStatus::Loading);

        let Ok(location) = self.location.read().map(|guard| guard.clone()) else {
            return;
        };

        let config = PollingConfig {
            poll_interval: self
                .poll_interval
                .read()
                .map(|guard| *guard)
                .unwrap_or(Duration::from_secs(600)),
            location,
            kind: self
                .provider_kind
                .read()
                .map(|guard| *guard)
                .unwrap_or_default(),
            visual_crossing_key: self
                .visual_crossing_key
                .read()
                .ok()
                .and_then(|guard| guard.clone()),
            weatherapi_key: self
                .weatherapi_key
                .read()
                .ok()
                .and_then(|guard| guard.clone()),
        };

        let new_token = self.cancellation_token.child_token();
        if let Ok(mut guard) = self.polling_token.write() {
            guard.cancel();
            polling::spawn(
                new_token.clone(),
                self.weather.clone(),
                self.status.clone(),
                config,
            );
            *guard = new_token;
        }
    }
}

impl Drop for WeatherService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
