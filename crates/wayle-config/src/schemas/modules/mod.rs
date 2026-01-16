mod battery;
mod clock;
mod media;

pub use battery::BatteryConfig;
pub use clock::ClockConfig;
pub use media::MediaConfig;

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
}
