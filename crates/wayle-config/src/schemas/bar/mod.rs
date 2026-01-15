use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Bar configuration.
#[wayle_config]
pub struct BarConfig {
    /// Per-monitor bar layouts.
    #[default(vec![BarLayout::default()])]
    pub layout: ConfigProperty<Vec<BarLayout>>,
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
    pub left: Vec<BarItem>,
    /// Modules in the center section.
    pub center: Vec<BarItem>,
    /// Modules in the right section.
    pub right: Vec<BarItem>,
}

impl Default for BarLayout {
    fn default() -> Self {
        Self {
            monitor: String::from("*"),
            extends: None,
            left: vec![BarItem::Module(BarModule::Dashboard)],
            center: vec![BarItem::Module(BarModule::Clock)],
            right: vec![BarItem::Module(BarModule::Systray)],
        }
    }
}

/// A bar item: either a standalone module or a named group of modules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum BarItem {
    /// A single module (e.g., "clock", "battery").
    Module(BarModule),
    /// A named group of modules with shared visual container.
    Group(BarGroup),
}

/// A named group of modules.
///
/// Groups provide visual containment via CSS. The group name becomes
/// a CSS ID selector (`#group-name`) for per-group styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BarGroup {
    /// Unique name for CSS targeting (becomes `#name` selector).
    pub name: String,
    /// Modules contained in this group.
    pub modules: Vec<BarModule>,
}

/// Available bar modules that can be placed in bar sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
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
