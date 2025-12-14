use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Configuration for the battery status module.
///
/// Controls the display and behavior of battery information in the status bar,
/// including percentage display and low battery warnings.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges)]
#[serde(default)]
pub struct BatteryConfig {
    /// Whether the battery module is displayed in the status bar.
    pub enabled: ConfigProperty<bool>,

    /// Whether to show the battery percentage alongside the icon.
    pub show_percentage: ConfigProperty<bool>,

    /// Battery percentage threshold for triggering a low battery warning.
    pub battery_warning: ConfigProperty<u8>,
}

impl Default for BatteryConfig {
    fn default() -> Self {
        Self {
            enabled: ConfigProperty::new(true),
            show_percentage: ConfigProperty::new(true),
            battery_warning: ConfigProperty::new(20),
        }
    }
}
