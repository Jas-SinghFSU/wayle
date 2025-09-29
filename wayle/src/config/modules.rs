use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{battery::BatteryConfig, clock::ClockConfig};

/// Configuration container for all available Wayle modules.
///
/// Holds optional configuration for each module. If a module's configuration
/// is not specified (None), the module uses its default settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct ModulesConfig {
    /// Configuration for the battery status module.
    pub battery: Option<BatteryConfig>,
    /// Configuration for the clock display module.
    pub clock: Option<ClockConfig>,
}
