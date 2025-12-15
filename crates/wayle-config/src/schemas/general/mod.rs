use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// General configuration settings for the Wayle application.
///
/// Contains global settings that affect the overall behavior of the application.
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
pub struct GeneralConfig {
    /// Temporary placeholder property
    pub temp: ConfigProperty<bool>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            temp: ConfigProperty::new(true),
        }
    }
}
