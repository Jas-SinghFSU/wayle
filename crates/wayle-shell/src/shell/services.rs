use std::sync::Arc;

use wayle_audio::AudioService;
use wayle_battery::BatteryService;
use wayle_bluetooth::BluetoothService;
use wayle_config::ConfigService;
use wayle_hyprland::HyprlandService;
use wayle_media::MediaService;
use wayle_network::NetworkService;
use wayle_notification::NotificationService;
use wayle_sysinfo::SysinfoService;
use wayle_systray::SystemTrayService;
use wayle_wallpaper::WallpaperService;
use wayle_weather::WeatherService;

use crate::services::IdleInhibitService;

/// Container for services used by shell components.
///
/// Services are injected at the composition root (bootstrap) and flow through
/// the component hierarchy. Optional services may be `None` when hardware,
/// compositor, or D-Bus daemons are unavailable - components gracefully
/// degrade in these cases.
#[derive(Clone)]
pub(crate) struct ShellServices {
    pub audio: Option<Arc<AudioService>>,
    pub battery: Option<Arc<BatteryService>>,
    pub bluetooth: Option<Arc<BluetoothService>>,
    pub config: Arc<ConfigService>,
    pub hyprland: Option<Arc<HyprlandService>>,
    pub idle_inhibit: Arc<IdleInhibitService>,
    pub media: Option<Arc<MediaService>>,
    pub network: Option<Arc<NetworkService>>,
    pub notification: Option<Arc<NotificationService>>,
    pub sysinfo: Arc<SysinfoService>,
    pub systray: Option<Arc<SystemTrayService>>,
    pub wallpaper: Option<Arc<WallpaperService>>,
    pub weather: Arc<WeatherService>,
}
