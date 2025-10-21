use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::Property;
use wayle_derive::{SubscribeChanges, UpdateFromToml};

/// Core clock functionality settings.
///
/// Each field is reactive and can be watched for changes.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, UpdateFromToml, SubscribeChanges)]
#[serde(default)]
pub struct ClockGeneralConfig {
    /// Time format string using strftime syntax.
    pub format: Property<String>,
}

impl Default for ClockGeneralConfig {
    fn default() -> Self {
        Self {
            format: Property::new(String::from("%H:%M")),
        }
    }
}
