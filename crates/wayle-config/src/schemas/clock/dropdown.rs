use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Configuration for the clock's dropdown menu.
///
/// Controls the content and behavior of the dropdown that appears
/// when clicking on the clock module.
/// Each field is reactive and can be watched for changes.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct ClockDropdownConfig {
    /// Whether to display a calendar widget in the dropdown menu.
    pub show_calendar: ConfigProperty<bool>,
}

impl Default for ClockDropdownConfig {
    fn default() -> Self {
        Self {
            show_calendar: ConfigProperty::new(true),
        }
    }
}
