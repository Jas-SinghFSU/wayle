use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Bar configuration.
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
pub struct BarConfig {
    /// Per-monitor bar layouts.
    pub layout: ConfigProperty<Vec<BarLayout>>,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            layout: ConfigProperty::new(vec![BarLayout::default()]),
        }
    }
}

/// Layout configuration for a bar on a specific monitor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(default)]
pub struct BarLayout {
    /// Monitor connector name (e.g., "DP-1") or "*" for all monitors.
    pub monitor: String,
    /// Inherit layout from another named layout.
    pub extends: Option<String>,
    /// Modules in the left section.
    pub left: Option<Vec<BarModule>>,
    /// Modules in the center-left section.
    pub center_left: Option<Vec<BarModule>>,
    /// Modules in the center section.
    pub center: Option<Vec<BarModule>>,
    /// Modules in the center-right section.
    pub center_right: Option<Vec<BarModule>>,
    /// Modules in the right section.
    pub right: Option<Vec<BarModule>>,
}

impl Default for BarLayout {
    fn default() -> Self {
        Self {
            monitor: String::from("*"),
            extends: None,
            left: None,
            center_left: None,
            center: None,
            center_right: None,
            right: None,
        }
    }
}

/// Available bar modules that can be placed in bar sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BarModule {
    /// Battery status and percentage.
    Battery,
    /// Bluetooth connection status and devices.
    Bluetooth,
    /// Current time display.
    Clock,
    /// CPU usage indicator.
    Cpu,
    /// Quick access dashboard button.
    Dashboard,
    /// Hyprland submap indicator.
    HyprlandSubmap,
    /// Hyprland workspace switcher.
    HyprlandWorkspaces,
    /// Hypridle status indicator.
    Hypridle,
    /// Hyprsunset (night light) toggle.
    Hyprsunset,
    /// Keyboard layout indicator.
    KeyboardInput,
    /// Media player controls.
    Media,
    /// Microphone mute status.
    Microphone,
    /// Network connection status.
    Network,
    /// Notification center button.
    Notifications,
    /// Power menu button.
    Power,
    /// RAM usage indicator.
    Ram,
    /// Visual separator between modules.
    Separator,
    /// Storage usage indicator.
    Storage,
    /// System tray icons.
    Systray,
    /// System updates indicator.
    Updates,
    /// Volume control.
    Volume,
    /// Weather conditions display.
    Weather,
    /// Active window title.
    WindowTitle,
    /// World clock with multiple timezones.
    WorldClock,
}
