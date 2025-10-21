use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::Property;
use wayle_derive::{SubscribeChanges, UpdateFromToml};

/// General configuration settings for the Wayle application.
///
/// Contains global settings that affect the overall behavior of the application.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, UpdateFromToml, SubscribeChanges)]
#[serde(default)]
pub struct GeneralConfig {
    /// Temporary placeholder property
    pub temp: Property<bool>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            temp: Property::new(true),
        }
    }
}
