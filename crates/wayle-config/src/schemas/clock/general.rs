use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Core clock functionality settings.
///
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
pub struct ClockGeneralConfig {
    /// Time format string using strftime syntax.
    pub format: ConfigProperty<String>,
}

impl Default for ClockGeneralConfig {
    fn default() -> Self {
        Self {
            format: ConfigProperty::new(String::from("%H:%M")),
        }
    }
}
