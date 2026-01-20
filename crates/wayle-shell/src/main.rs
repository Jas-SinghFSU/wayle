//! Wayle desktop shell - a GTK4/Relm4 status bar for Wayland compositors.

use std::error::Error;

use relm4::RelmApp;
use tracing::info;
use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_common::services::{self, ServiceRegistry};
use wayle_config::ConfigService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_systray::{SystemTrayService, types::TrayMode};
use wayle_wallpaper::WallpaperService;

mod shell;
mod startup;
mod tracing_init;
mod watchers;

use shell::Shell;
use startup::StartupTimer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_init::init()?;

    let timer = StartupTimer::new();

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

    timer.finish();

    let app = RelmApp::new("com.wayle.shell").visible_on_activate(false);
    app.run::<Shell>(());

    info!("Wayle shell stopped");
    Ok(())
}
