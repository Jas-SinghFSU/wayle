//! Weather service hot-reload watcher.

use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use wayle_common::services;
use wayle_config::{
    ConfigService,
    schemas::modules::{TemperatureUnit, WeatherConfig, WeatherProvider},
    secrets,
};
use wayle_weather::WeatherService;

pub fn spawn() {
    let Some(_) = services::try_get::<WeatherService>() else {
        return;
    };

    let config_service = services::get::<ConfigService>();
    let config = config_service.config().clone();
    let weather = &config.modules.weather;

    spawn_location_watcher(weather);
    spawn_provider_watcher(weather);
    spawn_units_watcher(weather);
    spawn_interval_watcher(weather);
    spawn_visual_crossing_key_watcher(weather);
    spawn_weatherapi_key_watcher(weather);
    spawn_secrets_reload_watcher(&config_service, weather);
}

fn spawn_location_watcher(config: &WeatherConfig) {
    let mut stream = config.location.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(location) = stream.next().await {
            let weather = services::get::<WeatherService>();
            weather.set_location(parse_location(&location));
        }
    });
}

fn spawn_provider_watcher(config: &WeatherConfig) {
    let mut stream = config.provider.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(provider) = stream.next().await {
            let weather = services::get::<WeatherService>();
            let kind = match provider {
                WeatherProvider::OpenMeteo => wayle_weather::WeatherProviderKind::OpenMeteo,
                WeatherProvider::VisualCrossing => {
                    wayle_weather::WeatherProviderKind::VisualCrossing
                }
                WeatherProvider::WeatherApi => wayle_weather::WeatherProviderKind::WeatherApi,
            };
            weather.set_provider(kind);
        }
    });
}

fn spawn_units_watcher(config: &WeatherConfig) {
    let mut stream = config.units.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(units) = stream.next().await {
            let weather = services::get::<WeatherService>();
            let units = match units {
                TemperatureUnit::Metric => wayle_weather::TemperatureUnit::Metric,
                TemperatureUnit::Imperial => wayle_weather::TemperatureUnit::Imperial,
            };
            weather.set_units(units);
        }
    });
}

fn spawn_interval_watcher(config: &WeatherConfig) {
    let mut stream = config.refresh_interval_seconds.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(interval_secs) = stream.next().await {
            let weather = services::get::<WeatherService>();
            weather.set_poll_interval(Duration::from_secs(u64::from(interval_secs)));
        }
    });
}

fn spawn_visual_crossing_key_watcher(config: &WeatherConfig) {
    let mut stream = config.visual_crossing_key.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(key) = stream.next().await {
            let weather = services::get::<WeatherService>();
            weather.set_visual_crossing_key(secrets::resolve(key));
        }
    });
}

fn spawn_weatherapi_key_watcher(config: &WeatherConfig) {
    let mut stream = config.weatherapi_key.watch();

    tokio::spawn(async move {
        stream.next().await;

        while let Some(key) = stream.next().await {
            let weather = services::get::<WeatherService>();
            weather.set_weatherapi_key(secrets::resolve(key));
        }
    });
}

fn spawn_secrets_reload_watcher(
    config_service: &Arc<ConfigService>,
    weather_config: &WeatherConfig,
) {
    let Some(mut rx) = config_service.subscribe_secrets_reload() else {
        return;
    };

    let vc_key = weather_config.visual_crossing_key.clone();
    let wa_key = weather_config.weatherapi_key.clone();

    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let weather = services::get::<WeatherService>();
            weather.set_visual_crossing_key(secrets::resolve(vc_key.get()));
            weather.set_weatherapi_key(secrets::resolve(wa_key.get()));
        }
    });
}

fn parse_location(location: &str) -> wayle_weather::LocationQuery {
    location
        .split_once(',')
        .and_then(|(lat, lon)| {
            let lat = lat.trim().parse().ok()?;
            let lon = lon.trim().parse().ok()?;
            Some(wayle_weather::LocationQuery::coords(lat, lon))
        })
        .unwrap_or_else(|| wayle_weather::LocationQuery::city(location))
}
