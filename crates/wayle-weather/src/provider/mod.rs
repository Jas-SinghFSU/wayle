mod open_meteo;
mod visual_crossing;
mod weatherapi;

use async_trait::async_trait;
pub use open_meteo::OpenMeteo;
pub use visual_crossing::VisualCrossing;
pub use weatherapi::WeatherApi;

use crate::{
    error::{Error, Result},
    model::{LocationQuery, Weather, WeatherProviderKind},
};

/// Trait for weather data providers.
///
/// Each provider implementation fetches weather data from a specific API
/// and normalizes it into the common `Weather` model.
#[async_trait]
pub trait WeatherProvider: Send + Sync {
    /// Returns the provider kind.
    fn kind(&self) -> WeatherProviderKind;

    /// Fetches weather data for the given location.
    ///
    /// # Errors
    ///
    /// Returns error on network failure, invalid location, or API issues.
    async fn fetch(&self, location: &LocationQuery) -> Result<Weather>;
}

/// Configuration for creating a weather provider.
pub struct ProviderConfig<'a> {
    /// Which provider to instantiate.
    pub kind: WeatherProviderKind,
    /// API key for Visual Crossing (required if `kind` is `VisualCrossing`).
    pub visual_crossing_key: Option<&'a str>,
    /// API key for WeatherAPI.com (required if `kind` is `WeatherApi`).
    pub weatherapi_key: Option<&'a str>,
}

/// Creates a weather provider from configuration.
///
/// # Errors
///
/// Returns `Error::ApiKeyMissing` if the provider requires an API key but none is provided.
pub fn create_provider(config: ProviderConfig<'_>) -> Result<Box<dyn WeatherProvider>> {
    match config.kind {
        WeatherProviderKind::OpenMeteo => Ok(Box::new(OpenMeteo::new())),
        WeatherProviderKind::VisualCrossing => {
            let key = config.visual_crossing_key.ok_or(Error::ApiKeyMissing {
                provider: "visual-crossing",
            })?;
            Ok(Box::new(VisualCrossing::new(key)))
        }
        WeatherProviderKind::WeatherApi => {
            let key = config.weatherapi_key.ok_or(Error::ApiKeyMissing {
                provider: "weatherapi",
            })?;
            Ok(Box::new(WeatherApi::new(key)))
        }
    }
}
