use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Styling configuration for the clock module.
///
/// Controls the visual appearance of the clock in both the status bar
/// and dropdown views, including colors, fonts, and icons.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct ClockStyling {
    /// Styling options for the clock button in the bar.
    pub button: ClockButtonStyling,

    /// Styling options for the clock dropdown panel.
    pub dropdown: ClockDropdownStyling,
}

/// Styling configuration for the clock button in the status bar.
///
/// Defines visual properties specific to how the clock appears when
/// displayed as a button in the main status bar.
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
pub struct ClockButtonStyling {
    /// CSS color of the clock icon in the bar button.
    pub icon: ConfigProperty<String>,
}

impl Default for ClockButtonStyling {
    fn default() -> Self {
        Self {
            icon: ConfigProperty::new(String::from("red")),
        }
    }
}

/// Styling configuration for the clock dropdown view.
///
/// Controls the visual appearance of the clock when displayed in the
/// dropdown panel, including calendar and time display styling.
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
pub struct ClockDropdownStyling {
    /// CSS color of the clock display in the dropdown panel.
    pub clock: ConfigProperty<String>,
}

impl Default for ClockDropdownStyling {
    fn default() -> Self {
        Self {
            clock: ConfigProperty::new(String::from("red")),
        }
    }
}
