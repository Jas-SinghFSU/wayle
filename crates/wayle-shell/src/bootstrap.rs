//! Application bootstrap: service initialization and instance detection.

use std::{error::Error, sync::Arc, time::Duration};

use tracing::warn;
use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_common::shell::APP_ID;
use wayle_config::{ConfigService, infrastructure::schema, secrets};
use wayle_hyprland::HyprlandService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_notification::NotificationService;
use wayle_sysinfo::SysinfoService;
use wayle_systray::{SystemTrayService, types::TrayMode};
use wayle_wallpaper::WallpaperService;
use wayle_weather::{LocationQuery, TemperatureUnit, WeatherProviderKind, WeatherService};
use zbus::{Connection, fdo::DBusProxy};

use crate::{services::IdleInhibitService, shell::ShellServices, startup::StartupTimer};

macro_rules! try_service {
    ($timer:expr, $name:literal, $future:expr) => {
        match $timer.time($name, $future).await {
            Ok(service) => Some(Arc::new(service)),
            Err(e) => {
                warn!(error = %e, concat!($name, " unavailable"));
                None
            }
        }
    };
    ($timer:expr, $name:literal, $future:expr, no_wrap) => {
        match $timer.time($name, $future).await {
            Ok(service) => Some(service),
            Err(e) => {
                warn!(error = %e, concat!($name, " unavailable"));
                None
            }
        }
    };
}

struct CoreServices {
    battery: Option<Arc<BatteryService>>,
    idle_inhibit: Arc<IdleInhibitService>,
    network: Option<Arc<NetworkService>>,
    sysinfo: Arc<SysinfoService>,
    wallpaper: Option<Arc<WallpaperService>>,
}

struct DaemonServices {
    audio: Option<Arc<AudioService>>,
    media: Option<Arc<MediaService>>,
    notification: Option<Arc<NotificationService>>,
    systray: Option<Arc<SystemTrayService>>,
}

struct OptionalServices {
    bluetooth: Option<Arc<BluetoothService>>,
    hyprland: Option<Arc<HyprlandService>>,
}

pub async fn is_already_running() -> bool {
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

pub async fn init_services() -> Result<(StartupTimer, ShellServices), Box<dyn Error>> {
    let mut timer = StartupTimer::new();

    if let Err(e) = timer
        .time("Schema", async { schema::ensure_schema_current() })
        .await
    {
        warn!(error = %e, "Could not write schema file");
    }

    let config_service = timer.time("Config", ConfigService::load()).await?;

    let (weather, core, daemons) = {
        let modules_config = &config_service.config().modules;
        let weather = timer.time_sync("Weather", || build_weather_service(modules_config));
        let core = init_core_services(&mut timer, modules_config).await?;
        let daemons = init_daemon_services(&mut timer, modules_config).await;
        (weather, core, daemons)
    };

    let optional = init_optional_services(&mut timer).await;

    timer.mark_services_done();

    let services = ShellServices {
        audio: daemons.audio,
        battery: core.battery,
        bluetooth: optional.bluetooth,
        config: config_service,
        hyprland: optional.hyprland,
        idle_inhibit: core.idle_inhibit,
        media: daemons.media,
        network: core.network,
        notification: daemons.notification,
        sysinfo: core.sysinfo,
        systray: daemons.systray,
        wallpaper: core.wallpaper,
        weather,
    };

    Ok((timer, services))
}

#[allow(clippy::cognitive_complexity)]
async fn init_core_services(
    timer: &mut StartupTimer,
    modules: &wayle_config::schemas::modules::ModulesConfig,
) -> Result<CoreServices, Box<dyn Error>> {
    let battery = try_service!(timer, "Battery", BatteryService::new());
    let network = try_service!(timer, "Network", NetworkService::new());
    let wallpaper = try_service!(timer, "Wallpaper", WallpaperService::new(), no_wrap);

    let sysinfo = Arc::new(timer.time_sync("Sysinfo", || {
        SysinfoService::builder()
            .cpu_interval(Duration::from_millis(modules.cpu.poll_interval_ms.get()))
            .memory_interval(Duration::from_millis(modules.ram.poll_interval_ms.get()))
            .disk_interval(Duration::from_millis(
                modules.storage.poll_interval_ms.get(),
            ))
            .network_interval(Duration::from_millis(
                modules.netstat.poll_interval_ms.get(),
            ))
            .build()
    }));

    let idle_inhibit = Arc::new(
        timer
            .time(
                "IdleInhibit",
                IdleInhibitService::new(modules.idle_inhibit.startup_duration.get()),
            )
            .await?,
    );

    Ok(CoreServices {
        battery,
        idle_inhibit,
        network,
        sysinfo,
        wallpaper,
    })
}

async fn init_optional_services(timer: &mut StartupTimer) -> OptionalServices {
    let bluetooth = try_service!(timer, "Bluetooth", BluetoothService::new());
    let hyprland = timer.time("Hyprland", HyprlandService::new()).await.ok();

    OptionalServices {
        bluetooth,
        hyprland,
    }
}

#[allow(clippy::cognitive_complexity)]
async fn init_daemon_services(
    timer: &mut StartupTimer,
    modules: &wayle_config::schemas::modules::ModulesConfig,
) -> DaemonServices {
    let audio = try_service!(
        timer,
        "Audio",
        AudioService::builder().with_daemon().build(),
        no_wrap
    );

    let media = try_service!(
        timer,
        "Media",
        MediaService::builder()
            .with_daemon()
            .ignored_players(modules.media.players_ignored.get().clone())
            .priority_players(modules.media.player_priority.get().clone())
            .build(),
        no_wrap
    );

    let notification = try_service!(
        timer,
        "Notification",
        NotificationService::builder().with_daemon().build(),
        no_wrap
    );

    let systray = try_service!(
        timer,
        "SystemTray",
        SystemTrayService::builder()
            .with_daemon()
            .mode(TrayMode::Auto)
            .build(),
        no_wrap
    );

    DaemonServices {
        audio,
        media,
        notification,
        systray,
    }
}

fn build_weather_service(
    modules: &wayle_config::schemas::modules::ModulesConfig,
) -> Arc<WeatherService> {
    use wayle_config::schemas::modules::{TemperatureUnit as CfgTempUnit, WeatherProvider};

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
