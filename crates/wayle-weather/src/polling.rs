use std::{sync::Arc, time::Duration};

use tokio::time::{interval, sleep};
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};
use wayle_common::Property;

use crate::{
    error::{Error, error_chain},
    model::{LocationQuery, Weather, WeatherProviderKind},
    provider::{ProviderConfig, create_provider},
};

const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(5);
const RATE_LIMIT_DELAY: Duration = Duration::from_secs(60);

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
                    debug!("weather polling stopped");
                    return;
                }
                _ = ticker.tick() => {
                    fetch_with_retry(&token, &weather, &config).await;
                }
            }
        }
    });
}

async fn fetch_with_retry(
    token: &CancellationToken,
    weather: &Property<Option<Arc<Weather>>>,
    config: &PollingConfig,
) {
    let provider = match create_provider(ProviderConfig {
        kind: config.kind,
        visual_crossing_key: config.visual_crossing_key.as_deref(),
        weatherapi_key: config.weatherapi_key.as_deref(),
    }) {
        Ok(p) => p,
        Err(err) => {
            warn!(error = %error_chain(&err), "cannot create weather provider");
            return;
        }
    };

    for attempt in 1..=MAX_RETRIES {
        match provider.fetch(&config.location).await {
            Ok(data) => {
                on_fetch_success(weather, data);
                return;
            }
            Err(err) if err.is_retryable() && attempt < MAX_RETRIES => {
                if wait_before_retry(token, &err, attempt).await.is_err() {
                    return;
                }
            }
            Err(err) => {
                warn!(error = %error_chain(&err), "cannot fetch weather data");
                return;
            }
        }
    }
}

fn on_fetch_success(weather: &Property<Option<Arc<Weather>>>, data: Weather) {
    debug!(
        city = %data.location.city,
        temp = %data.current.temperature,
        "weather updated"
    );
    weather.set(Some(Arc::new(data)));
}

async fn wait_before_retry(token: &CancellationToken, err: &Error, attempt: u32) -> Result<(), ()> {
    let delay = retry_delay(err, attempt);
    debug!(
        error = %error_chain(err),
        attempt,
        delay_ms = delay.as_millis(),
        "weather fetch failed, retrying"
    );

    tokio::select! {
        () = token.cancelled() => Err(()),
        () = sleep(delay) => Ok(()),
    }
}

fn retry_delay(err: &Error, attempt: u32) -> Duration {
    if matches!(err, Error::RateLimited { .. }) {
        return RATE_LIMIT_DELAY;
    }
    INITIAL_RETRY_DELAY * 2u32.saturating_pow(attempt - 1)
}
