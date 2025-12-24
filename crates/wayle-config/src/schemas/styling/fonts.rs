use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Font configuration for the Wayle shell.
///
/// Controls the font families used throughout the interface.
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
pub struct FontConfig {
    /// Sans-serif font family for UI text and labels.
    pub sans: ConfigProperty<String>,

    /// Monospace font family for code and technical content.
    pub mono: ConfigProperty<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            sans: ConfigProperty::new(String::from("Inter")),
            mono: ConfigProperty::new(String::from("JetBrains Mono")),
        }
    }
}
