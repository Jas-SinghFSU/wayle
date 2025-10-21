use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_derive::{SubscribeChanges, UpdateFromToml};

use super::{battery::BatteryConfig, clock::ClockConfig};

/// Configuration container for all available Wayle modules.
///
/// If a module's configuration is not specified in TOML,
/// it uses its default settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, UpdateFromToml, SubscribeChanges)]
#[serde(default)]
pub struct ModulesConfig {
    /// Configuration for the battery status module.
    pub battery: BatteryConfig,
    /// Configuration for the clock display module.
    pub clock: ClockConfig,
}
