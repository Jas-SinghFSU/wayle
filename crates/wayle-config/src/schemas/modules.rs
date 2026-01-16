use wayle_derive::wayle_config;

use super::{battery::BatteryConfig, clock::ClockConfig, media::MediaConfig};

/// Configuration for all available Wayle modules.
#[wayle_config]
pub struct ModulesConfig {
    /// Configuration for the battery status module.
    pub battery: BatteryConfig,
    /// Configuration for the clock display module.
    pub clock: ClockConfig,
    /// Configuration for the media player module.
    pub media: MediaConfig,
}
