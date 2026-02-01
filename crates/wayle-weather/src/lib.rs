//! Weather data service with multi-provider support.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use wayle_weather::WeatherService;
//!
//! let weather = WeatherService::builder().build();
//!
//! if let Some(data) = weather.weather.get().as_ref() {
//!     let temp = data.current.temperature.celsius();
//!     let cond = &data.current.condition;
//!     println!("{temp}°C, {cond:?}");
//! }
//! ```
//!
//! # Watching for Changes
//!
//! ```rust,no_run
//! use wayle_weather::WeatherService;
//! use tokio_stream::StreamExt;
//!
//! # let weather = WeatherService::builder().build();
//! # async fn watch(weather: WeatherService) {
//! let mut stream = weather.weather.watch();
//! while let Some(data) = stream.next().await {
//!     if let Some(w) = data.as_ref() {
//!         println!("Updated: {}°C", w.current.temperature.celsius());
//!     }
//! }
//! # }
//! ```
//!
//! # Configuration
//!
//! | Method | Effect |
//! |--------|--------|
//! | `poll_interval(Duration)` | How often to fetch fresh data |
//! | `provider(WeatherProviderKind)` | Which weather API to use |
//! | `location(LocationQuery)` | City or coordinates for forecasts |
//! | `units(TemperatureUnit)` | Celsius or Fahrenheit display |
//! | `visual_crossing_key(key)` | API key for Visual Crossing |
//! | `weatherapi_key(key)` | API key for WeatherAPI.com |
//!
//! ```rust,no_run
//! use wayle_weather::{WeatherService, WeatherProviderKind, LocationQuery, TemperatureUnit};
//! use std::time::Duration;
//!
//! let weather = WeatherService::builder()
//!     .provider(WeatherProviderKind::OpenMeteo)
//!     .location(LocationQuery::city("Tokyo"))
//!     .units(TemperatureUnit::Metric)
//!     .poll_interval(Duration::from_secs(15 * 60))
//!     .build();
//! ```
//!
//! # Providers
//!
//! | Provider | API Key |
//! |----------|---------|
//! | [`OpenMeteo`](WeatherProviderKind::OpenMeteo) | No |
//! | [`VisualCrossing`](WeatherProviderKind::VisualCrossing) | Yes |
//! | [`WeatherApi`](WeatherProviderKind::WeatherApi) | Yes |
//!
//! # Reactive Properties
//!
//! The `weather` field is a [`Property<Option<Arc<Weather>>>`](wayle_common::Property):
//! - `.get()` - Current value snapshot
//! - `.watch()` - Stream yielding on changes
//!
//! # Service Fields
//!
//! | Field | Type | Description |
//! |-------|------|-------------|
//! | `weather` | `Option<Arc<Weather>>` | Latest weather data, `None` until first fetch |
//!
//! # Runtime Updates
//!
//! All settings can change after creation:
//!
//! - [`set_poll_interval()`](WeatherService::set_poll_interval) - Polling frequency
//! - [`set_location()`](WeatherService::set_location) - Weather location
//! - [`set_units()`](WeatherService::set_units) - Temperature display
//! - [`set_provider()`](WeatherService::set_provider) - Weather source
//!
//! # Weather Data
//!
//! The [`Weather`] struct contains:
//! - `current` - Real-time conditions ([`CurrentWeather`])
//! - `hourly` - Next 24+ hours ([`HourlyForecast`])
//! - `daily` - Next 7+ days ([`DailyForecast`])
//! - `location` - Resolved coordinates
//! - `astronomy` - Sunrise/sunset times

mod builder;

/// Weather service error types and result aliases.
pub mod error;

/// Weather data models including forecasts, conditions, and location.
pub mod model;

pub(crate) mod polling;

/// Weather data provider implementations and configuration.
pub mod provider;

mod service;

/// Domain-specific measurement types for weather data.
pub mod types;

pub use builder::WeatherServiceBuilder;
pub use error::{Error, Result};
pub use model::{
    Astronomy, CurrentWeather, DailyForecast, HourlyForecast, Location, LocationQuery,
    TemperatureUnit, Weather, WeatherCondition, WeatherProviderKind,
};
pub use provider::{ProviderConfig, WeatherProvider, create_provider};
pub use service::WeatherService;
pub use types::{
    Distance, Percentage, Precipitation, Pressure, Speed, Temperature, UvIndex, WindDirection,
};
