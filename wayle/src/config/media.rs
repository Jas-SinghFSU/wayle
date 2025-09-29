use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Media service configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct MediaConfig {
    /// List of player bus name patterns to ignore during discovery
    pub ignored_players: Vec<String>,

    /// Whether the media module is displayed in the status bar.
    pub enabled: bool,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            ignored_players: Vec::new(),
            enabled: true,
        }
    }
}
