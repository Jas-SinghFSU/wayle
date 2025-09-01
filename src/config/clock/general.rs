use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Core clock functionality settings.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ClockGeneralConfig {
    /// Time format string using strftime syntax.
    pub format: String,
}

impl Default for ClockGeneralConfig {
    fn default() -> Self {
        Self {
            format: String::from("%H:%M"),
        }
    }
}
