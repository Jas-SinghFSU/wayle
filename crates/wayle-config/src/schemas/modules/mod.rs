mod battery;
mod bluetooth;
mod clock;
mod cpu;
mod dashboard;
mod media;
mod microphone;
mod netstat;
mod network;
mod notification;
mod ram;
mod storage;
mod systray;
mod volume;

pub use battery::BatteryConfig;
pub use bluetooth::BluetoothConfig;
pub use clock::ClockConfig;
pub use cpu::CpuConfig;
pub use dashboard::DashboardConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
pub use microphone::MicrophoneConfig;
pub use netstat::NetstatConfig;
pub use network::NetworkConfig;
pub use notification::NotificationConfig;
pub use ram::RamConfig;
pub use storage::StorageConfig;
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
    /// CPU usage module.
    pub cpu: CpuConfig,
    /// Dashboard module.
    pub dashboard: DashboardConfig,
    /// Media player module.
    pub media: MediaConfig,
    /// Microphone input module.
    pub microphone: MicrophoneConfig,
    /// Network connection module.
    pub network: NetworkConfig,
    /// Network traffic statistics module.
    pub netstat: NetstatConfig,
    /// Notification center module.
    pub notification: NotificationConfig,
    /// RAM usage module.
    pub ram: RamConfig,
    /// Storage usage module.
    pub storage: StorageConfig,
    /// System tray module.
    pub systray: SystrayConfig,
    /// Volume control module.
    pub volume: VolumeConfig,
}
