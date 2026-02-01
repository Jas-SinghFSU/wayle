mod battery;
mod bluetooth;
mod clock;
mod cpu;
mod dashboard;
mod keyboard_input;
mod media;
mod microphone;
mod netstat;
mod network;
mod notification;
mod power;
mod ram;
mod separator;
mod storage;
mod systray;
mod volume;
mod weather;
mod world_clock;

pub use battery::BatteryConfig;
pub use bluetooth::BluetoothConfig;
pub use clock::ClockConfig;
pub use cpu::CpuConfig;
pub use dashboard::DashboardConfig;
pub use keyboard_input::KeyboardInputConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
pub use microphone::MicrophoneConfig;
pub use netstat::NetstatConfig;
pub use network::NetworkConfig;
pub use notification::NotificationConfig;
pub use power::PowerConfig;
pub use ram::RamConfig;
pub use separator::SeparatorConfig;
pub use storage::StorageConfig;
pub use systray::{SystrayConfig, TrayItemOverride};
pub use volume::VolumeConfig;
use wayle_derive::wayle_config;
pub use weather::{TemperatureUnit, WeatherConfig, WeatherProvider};
pub use world_clock::WorldClockConfig;

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
    /// Keyboard input module.
    #[serde(rename = "keyboard-input")]
    pub keyboard_input: KeyboardInputConfig,
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
    /// Power menu module.
    pub power: PowerConfig,
    /// RAM usage module.
    pub ram: RamConfig,
    /// Storage usage module.
    pub storage: StorageConfig,
    /// Separator module.
    pub separator: SeparatorConfig,
    /// System tray module.
    pub systray: SystrayConfig,
    /// Volume control module.
    pub volume: VolumeConfig,
    /// Weather display module.
    pub weather: WeatherConfig,
    /// World clock module.
    #[serde(rename = "world-clock")]
    pub world_clock: WorldClockConfig,
}
