use std::{sync::Arc, time::Duration};

use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::Property;

use crate::{
    model::{LocationQuery, Weather, WeatherProviderKind},
    provider::{ProviderConfig, create_provider},
};

pub(crate) struct PollingConfig {
    pub kind: WeatherProviderKind,
    pub visual_crossing_key: Option<String>,
    pub weatherapi_key: Option<String>,
    pub location: LocationQuery,
    pub poll_interval: Duration,
}

pub(crate) fn spawn(
    token: CancellationToken,
    weather: Property<Option<Arc<Weather>>>,
    config: PollingConfig,
) {
    tokio::spawn(async move {
        let mut ticker = interval(config.poll_interval);

        loop {
            tokio::select! {
                () = token.cancelled() => {
                    debug!("Weather polling cancelled");
                    return;
                }
                _ = ticker.tick() => {
                    let provider = match create_provider(ProviderConfig {
                        kind: config.kind,
                        visual_crossing_key: config.visual_crossing_key.as_deref(),
                        weatherapi_key: config.weatherapi_key.as_deref(),
                    }) {
                        Ok(p) => p,
                        Err(err) => {
                            warn!(error = %err, "cannot create weather provider");
                            continue;
                        }
                    };

                    match provider.fetch(&config.location).await {
                        Ok(data) => {
                            debug!(
                                city = %data.location.city,
                                temp = %data.current.temperature,
                                "Weather data updated"
                            );
                            weather.set(Some(Arc::new(data)));
                        }
                        Err(err) => {
                            warn!(error = %err, "cannot fetch weather data");
                        }
                    }
                }
            }
        }
    });
}
