use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::Property;
use wayle_derive::{SubscribeChanges, UpdateFromToml};

/// Configuration for the battery status module.
///
/// Controls the display and behavior of battery information in the status bar,
/// including percentage display and low battery warnings.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, UpdateFromToml, SubscribeChanges)]
#[serde(default)]
pub struct BatteryConfig {
    /// Whether the battery module is displayed in the status bar.
    pub enabled: Property<bool>,

    /// Whether to show the battery percentage alongside the icon.
    pub show_percentage: Property<bool>,

    /// Battery percentage threshold for triggering a low battery warning.
    pub battery_warning: Property<u8>,
}

impl Default for BatteryConfig {
    fn default() -> Self {
        Self {
            enabled: Property::new(true),
            show_percentage: Property::new(true),
            battery_warning: Property::new(20),
        }
    }
}
