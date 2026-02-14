use std::{sync::Arc, time::Duration};

use wayle_config::{
    schemas::modules::{ModulesConfig, TemperatureUnit as CfgTempUnit, WeatherProvider},
    secrets,
};
use wayle_weather::{LocationQuery, TemperatureUnit, WeatherProviderKind, WeatherService};

pub(super) fn build_weather_service(modules: &ModulesConfig) -> Arc<WeatherService> {
    let cfg = &modules.weather;

    let location = parse_location(cfg.location.get().as_str());
    let provider = match cfg.provider.get() {
        WeatherProvider::OpenMeteo => WeatherProviderKind::OpenMeteo,
        WeatherProvider::VisualCrossing => WeatherProviderKind::VisualCrossing,
        WeatherProvider::WeatherApi => WeatherProviderKind::WeatherApi,
    };
    let units = match cfg.units.get() {
        CfgTempUnit::Metric => TemperatureUnit::Metric,
        CfgTempUnit::Imperial => TemperatureUnit::Imperial,
    };
    let poll_interval = Duration::from_secs(u64::from(cfg.refresh_interval_seconds.get()));

    let mut builder = WeatherService::builder()
        .poll_interval(poll_interval)
        .provider(provider)
        .location(location)
        .units(units);

    if let Some(key) = secrets::resolve(cfg.visual_crossing_key.get()) {
        builder = builder.visual_crossing_key(key);
    }
    if let Some(key) = secrets::resolve(cfg.weatherapi_key.get()) {
        builder = builder.weatherapi_key(key);
    }

    Arc::new(builder.build())
}

fn parse_location(location: &str) -> LocationQuery {
    location
        .split_once(',')
        .and_then(|(lat, lon)| {
            let lat = lat.trim().parse().ok()?;
            let lon = lon.trim().parse().ok()?;
            Some(LocationQuery::coords(lat, lon))
        })
        .unwrap_or_else(|| LocationQuery::city(location))
}
