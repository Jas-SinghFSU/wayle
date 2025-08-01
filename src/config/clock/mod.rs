mod button;
mod dropdown;
mod general;
mod styling;

use button::ClockButtonConfig;
use dropdown::ClockDropdownConfig;
use general::ClockGeneralConfig;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use styling::{ClockButtonStyling, ClockDropdownStyling, ClockStyling};

use crate::docs::{BehaviorConfigs, ModuleInfo, ModuleInfoProvider, StylingConfigs};

/// Configuration for the clock module.
///
/// Provides comprehensive settings for displaying time and calendar information,
/// including general behavior, button appearance, dropdown functionality, and styling options.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClockConfig {
    /// General configuration settings that apply to all clock functionality.
    #[serde(default)]
    pub general: ClockGeneralConfig,

    /// Settings specific to the clock's appearance in the status bar button.
    #[serde(default)]
    pub button: ClockButtonConfig,

    /// Configuration for the clock's dropdown panel behavior and content.
    #[serde(default)]
    pub dropdown: ClockDropdownConfig,

    /// Visual styling options for customizing the clock's appearance.
    #[serde(default)]
    pub styling: ClockStyling,
}

impl ModuleInfoProvider for ClockConfig {
    fn module_info() -> ModuleInfo {
        let behavior_configs: BehaviorConfigs = vec![
            ("general".to_string(), || schema_for!(ClockGeneralConfig)),
            ("button".to_string(), || schema_for!(ClockButtonConfig)),
            ("dropdown".to_string(), || schema_for!(ClockDropdownConfig)),
        ];

        let styling_configs: StylingConfigs = vec![
            ("button".to_string(), || schema_for!(ClockButtonStyling)),
            ("dropdown".to_string(), || schema_for!(ClockDropdownStyling)),
        ];

        ModuleInfo {
            name: "clock".to_string(),
            icon: "󰥔".to_string(),
            description: "Controls the clock display and calendar settings".to_string(),
            behavior_configs,
            styling_configs,
        }
    }
}
