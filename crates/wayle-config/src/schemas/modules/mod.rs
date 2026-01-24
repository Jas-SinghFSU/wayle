mod battery;
mod clock;
mod media;
mod volume;

pub use battery::BatteryConfig;
pub use clock::ClockConfig;
pub use media::{BUILTIN_MAPPINGS, MediaConfig, MediaIconType};
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
    /// Volume control module.
    pub volume: VolumeConfig,
}
