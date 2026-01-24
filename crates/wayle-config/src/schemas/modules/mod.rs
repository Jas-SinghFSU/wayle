mod battery;
mod clock;
mod media;
mod microphone;
mod volume;

pub use battery::BatteryConfig;
pub use clock::ClockConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
pub use microphone::MicrophoneConfig;
pub use volume::VolumeConfig;
use wayle_derive::wayle_config;

/// Configuration for all available Wayle modules.
#[wayle_config]
pub struct ModulesConfig {
    /// Battery status module.
    pub battery: BatteryConfig,
    /// Clock display module.
    pub clock: ClockConfig,
    /// Media player module.
    pub media: MediaConfig,
    /// Microphone input module.
    pub microphone: MicrophoneConfig,
    /// Volume control module.
    pub volume: VolumeConfig,
}
