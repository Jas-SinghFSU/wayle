use wayle_derive::wayle_config;

use super::{battery::BatteryConfig, clock::ClockConfig};

/// Configuration container for all available Wayle modules.
///
/// If a module's configuration is not specified in TOML,
/// it uses its default settings.
#[wayle_config]
pub struct ModulesConfig {
    /// Configuration for the battery status module.
    pub battery: BatteryConfig,
    /// Configuration for the clock display module.
    pub clock: ClockConfig,
}
