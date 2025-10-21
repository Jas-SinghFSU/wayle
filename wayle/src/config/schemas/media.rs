use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::Property;
use wayle_derive::{SubscribeChanges, UpdateFromToml};

/// Media service configuration.
///
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, UpdateFromToml, SubscribeChanges)]
#[serde(default)]
pub struct MediaConfig {
    /// List of player bus name patterns to ignore during discovery
    pub ignored_players: Property<Vec<String>>,

    /// Whether the media module is displayed in the status bar.
    pub enabled: Property<bool>,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            ignored_players: Property::new(Vec::new()),
            enabled: Property::new(true),
        }
    }
}
