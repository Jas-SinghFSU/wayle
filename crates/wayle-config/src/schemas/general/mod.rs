mod bar;

pub use bar::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// General configuration settings for the Wayle application.
///
/// Contains global settings that don't affect styling.
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
pub struct GeneralConfig {
    /// Bar configuration.
    pub bar: BarConfig,
}
