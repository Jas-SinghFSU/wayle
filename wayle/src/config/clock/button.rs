use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configuration for the clock's appearance in the status bar.
///
/// Controls visual elements specific to how the clock module appears
/// when displayed in the main status bar.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ClockButtonConfig {
    /// Whether to display a clock icon alongside the time text.
    pub show_icon: bool,
}

impl Default for ClockButtonConfig {
    fn default() -> Self {
        Self { show_icon: true }
    }
}
