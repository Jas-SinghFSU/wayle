//! Wayle desktop shell - a GTK4/Relm4 status bar for Wayland compositors.

use std::{error::Error, sync::Arc, time::Duration};

use relm4::RelmApp;
use tokio::runtime::Runtime;
use tracing::{info, warn};
use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_common::{services::ServiceRegistry, shell::APP_ID};
use wayle_config::{ConfigService, infrastructure::schema, secrets};
use wayle_hyprland::HyprlandService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_notification::NotificationService;
use wayle_sysinfo::SysinfoService;
use wayle_systray::{SystemTrayService, types::TrayMode};
use wayle_wallpaper::WallpaperService;
use wayle_weather::WeatherService;
use zbus::{Connection, fdo::DBusProxy};

mod i18n;
mod services;
mod shell;
mod startup;
mod tracing_init;
mod watchers;

use shell::Shell;
use startup::StartupTimer;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_init::init()?;

    let runtime = Runtime::new()?;

    let _guard = runtime.enter();

    if runtime.block_on(is_already_running()) {
        eprintln!("Wayle shell is already running");
        return Ok(());
    }

    let timer = runtime.block_on(init_services())?;

    let app = RelmApp::new("com.wayle.shell").visible_on_activate(false);
    app.run::<Shell>(timer);

    info!("Wayle shell stopped");

    runtime.shutdown_background();
    Ok(())
}

async fn is_already_running() -> bool {
    let Ok(connection) = Connection::session().await else {
        return false;
    };

    let Ok(dbus) = DBusProxy::new(&connection).await else {
        return false;
    };

    let Ok(name) = APP_ID.try_into() else {
        return false;
    };

    dbus.name_has_owner(name).await.unwrap_or(false)
}

async fn init_services() -> Result<StartupTimer, Box<dyn Error>> {
    let mut timer = StartupTimer::new();

    if let Err(e) = timer
        .time("Schema", async { schema::ensure_schema_current() })
        .await
    {
        warn!(error = %e, "Could not write schema file");
    }

    let mut registry = ServiceRegistry::new();

    registry.register_arc(
        timer
            .time("Audio", AudioService::builder().with_daemon().build())
            .await?,
    );
    let config_service = timer.time("Config", ConfigService::load()).await?;
    let modules_config = &config_service.config().modules;
    let media_config = &modules_config.media;
    let ignored_players = media_config.players_ignored.get().clone();
    let priority_players = media_config.player_priority.get().clone();
    let cpu_interval = Duration::from_millis(modules_config.cpu.poll_interval_ms.get());
    let memory_interval = Duration::from_millis(modules_config.ram.poll_interval_ms.get());
    let disk_interval = Duration::from_millis(modules_config.storage.poll_interval_ms.get());
    let network_interval = Duration::from_millis(modules_config.netstat.poll_interval_ms.get());
    let idle_startup_duration = modules_config.idle_inhibit.startup_duration.get();
    let weather_service = timer.time_sync("Weather", || build_weather_service(modules_config));
    registry.register_arc(config_service);

    registry.register(timer.time("Battery", BatteryService::new()).await?);
    registry.register(timer.time("Bluetooth", BluetoothService::new()).await?);

    if let Ok(hyprland) = timer.time("Hyprland", HyprlandService::new()).await {
        registry.register_arc(hyprland);
    }
    registry.register_arc(
        timer
            .time(
                "Media",
                MediaService::builder()
                    .with_daemon()
                    .ignored_players(ignored_players)
                    .priority_players(priority_players)
                    .build(),
            )
            .await?,
    );
    registry.register(timer.time("Network", NetworkService::new()).await?);
    registry.register_arc(
        timer
            .time(
                "Notification",
                NotificationService::builder().with_daemon().build(),
            )
            .await?,
    );
    registry.register_arc(
        timer
            .time(
                "SystemTray",
                SystemTrayService::builder()
                    .with_daemon()
                    .mode(TrayMode::Auto)
                    .build(),
            )
            .await?,
    );
    registry.register_arc(timer.time("Wallpaper", WallpaperService::new()).await?);
    registry.register_arc(weather_service);

    registry.register(timer.time_sync("Sysinfo", || {
        SysinfoService::builder()
            .cpu_interval(cpu_interval)
            .memory_interval(memory_interval)
            .disk_interval(disk_interval)
            .network_interval(network_interval)
            .build()
    }));

    registry.register(
        timer
            .time(
                "IdleInhibit",
                services::IdleInhibitService::new(idle_startup_duration),
            )
            .await?,
    );

    wayle_common::services::init(registry);
    timer.mark_services_done();

    Ok(timer)
}

fn build_weather_service(
    modules: &wayle_config::schemas::modules::ModulesConfig,
) -> Arc<WeatherService> {
    use wayle_config::schemas::modules::{TemperatureUnit, WeatherProvider};

    let cfg = &modules.weather;

    let location = parse_location(cfg.location.get().as_str());
    let provider = match cfg.provider.get() {
        WeatherProvider::OpenMeteo => wayle_weather::WeatherProviderKind::OpenMeteo,
        WeatherProvider::VisualCrossing => wayle_weather::WeatherProviderKind::VisualCrossing,
        WeatherProvider::WeatherApi => wayle_weather::WeatherProviderKind::WeatherApi,
    };
    let units = match cfg.units.get() {
        TemperatureUnit::Metric => wayle_weather::TemperatureUnit::Metric,
        TemperatureUnit::Imperial => wayle_weather::TemperatureUnit::Imperial,
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
