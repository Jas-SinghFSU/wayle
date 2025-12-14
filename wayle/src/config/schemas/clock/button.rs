use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Configuration for the clock's appearance in the status bar.
///
/// Controls visual elements specific to how the clock module appears
/// when displayed in the main status bar.
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges)]
#[serde(default)]
pub struct ClockButtonConfig {
    /// Whether to display a clock icon alongside the time text.
    pub show_icon: ConfigProperty<bool>,
}

impl Default for ClockButtonConfig {
    fn default() -> Self {
        Self {
            show_icon: ConfigProperty::new(true),
        }
    }
}
