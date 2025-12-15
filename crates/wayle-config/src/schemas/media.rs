use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Media service configuration.
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
pub struct MediaConfig {
    /// List of player bus name patterns to ignore during discovery
    pub ignored_players: ConfigProperty<Vec<String>>,

    /// Whether the media module is displayed in the status bar.
    pub enabled: ConfigProperty<bool>,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            ignored_players: ConfigProperty::new(Vec::new()),
            enabled: ConfigProperty::new(true),
        }
    }
}
