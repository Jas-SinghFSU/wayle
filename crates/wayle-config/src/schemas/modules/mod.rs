mod battery;
mod bluetooth;
mod clock;
mod dashboard;
mod media;
mod microphone;
mod network;
mod notification;
mod systray;
mod volume;

pub use battery::BatteryConfig;
pub use bluetooth::BluetoothConfig;
pub use clock::ClockConfig;
pub use dashboard::DashboardConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
pub use microphone::MicrophoneConfig;
pub use network::NetworkConfig;
pub use notification::NotificationConfig;
pub use systray::{SystrayConfig, TrayItemOverride};
pub use volume::VolumeConfig;
use wayle_derive::wayle_config;

/// Configuration for all available Wayle modules.
#[wayle_config]
pub struct ModulesConfig {
    /// Battery status module.
    pub battery: BatteryConfig,
    /// Bluetooth connection module.
    pub bluetooth: BluetoothConfig,
    /// Clock display module.
    pub clock: ClockConfig,
    /// Dashboard module.
    pub dashboard: DashboardConfig,
    /// Media player module.
    pub media: MediaConfig,
    /// Microphone input module.
    pub microphone: MicrophoneConfig,
    /// Network connection module.
    pub network: NetworkConfig,
    /// Notification center module.
    pub notification: NotificationConfig,
    /// System tray module.
    pub systray: SystrayConfig,
    /// Volume control module.
    pub volume: VolumeConfig,
}
