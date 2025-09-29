use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// General configuration settings for the Wayle application.
///
/// Contains global settings that affect the overall behavior of the application.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct GeneralConfig {
    /// Reserved for future general configuration options
    #[serde(skip)]
    _reserved: (),
}
