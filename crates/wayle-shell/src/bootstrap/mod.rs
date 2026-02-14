//! Application bootstrap: service initialization and instance detection.

mod wallpaper;
mod weather;

use std::{error::Error, sync::Arc, time::Duration};

use tracing::warn;
use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_common::shell::APP_ID;
use wayle_config::{ConfigService, infrastructure::schema, schemas::styling::ThemeProvider};
use wayle_hyprland::HyprlandService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_notification::NotificationService;
use wayle_sysinfo::SysinfoService;
use wayle_systray::{SystemTrayService, types::TrayMode};
use wayle_wallpaper::{WallpaperService, types::ColorExtractor};
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
        let config = config_service.config();
        let weather = timer.time_sync("Weather", || {
            weather::build_weather_service(&config.modules)
        });
        let core = init_core_services(&mut timer, config).await?;
        let daemons = init_daemon_services(&mut timer, &config.modules).await;
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
    config: &wayle_config::Config,
) -> Result<CoreServices, Box<dyn Error>> {
    let modules = &config.modules;

    let battery = try_service!(timer, "Battery", BatteryService::new());
    let network = try_service!(timer, "Network", NetworkService::new());
    let theming_monitor = config.styling.theming_monitor.get();
    let theming_monitor = if theming_monitor.is_empty() {
        None
    } else {
        Some(theming_monitor)
    };
    let color_extractor = match config.styling.theme_provider.get() {
        ThemeProvider::Wayle => ColorExtractor::None,
        ThemeProvider::Matugen => ColorExtractor::Matugen,
        ThemeProvider::Pywal => ColorExtractor::Pywal,
        ThemeProvider::Wallust => ColorExtractor::Wallust,
    };
    let wallpaper = try_service!(
        timer,
        "Wallpaper",
        wallpaper::build_wallpaper_service(&config.wallpaper, theming_monitor, color_extractor),
        no_wrap
    );

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
