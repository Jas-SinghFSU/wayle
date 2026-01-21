//! Wayle desktop shell - a GTK4/Relm4 status bar for Wayland compositors.

use std::error::Error;

use relm4::RelmApp;
use tokio::runtime::Runtime;
use tracing::info;
use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_common::{
    services::{self, ServiceRegistry},
    shell::APP_ID,
};
use wayle_config::ConfigService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_systray::{SystemTrayService, types::TrayMode};
use wayle_wallpaper::WallpaperService;
use zbus::{Connection, fdo::DBusProxy};

mod shell;
mod startup;
mod tracing_init;
mod watchers;

use shell::Shell;
use startup::StartupTimer;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_init::init()?;

    let runtime = Runtime::new()?;

    let _ = runtime.enter();

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

    let mut registry = ServiceRegistry::new();

    registry.register_arc(
        timer
            .time("Audio", AudioService::builder().with_daemon().build())
            .await?,
    );
    registry.register(timer.time("Battery", BatteryService::new()).await?);
    registry.register(timer.time("Bluetooth", BluetoothService::new()).await?);
    registry.register_arc(
        timer
            .time("Media", MediaService::builder().with_daemon().build())
            .await?,
    );
    registry.register(timer.time("Network", NetworkService::new()).await?);
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
    registry.register_arc(timer.time("Config", ConfigService::load()).await?);
    registry.register_arc(timer.time("Wallpaper", WallpaperService::new()).await?);

    services::init(registry);
    timer.mark_services_done();

    Ok(timer)
}
