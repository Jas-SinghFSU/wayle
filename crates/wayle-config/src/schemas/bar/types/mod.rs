mod shadow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use shadow::ShadowPreset;

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
    /// A single module.
    Module(BarModule),
    /// A named group of modules with shared visual container.
    Group(BarGroup),
}

/// Named group of modules. The name becomes a CSS ID selector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BarGroup {
    /// Unique name for CSS targeting (becomes `#name` selector).
    pub name: String,
    /// Modules contained in this group.
    pub modules: Vec<BarModule>,
}

/// Available bar modules.
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

/// Bar position on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Location {
    /// Top edge of the screen.
    Top,
    /// Bottom edge of the screen.
    Bottom,
    /// Left edge of the screen.
    Left,
    /// Right edge of the screen.
    Right,
}

impl Location {
    /// CSS class name for this location.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Whether this location results in a vertical bar layout.
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// Border placement for bar buttons.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum BorderLocation {
    /// No border.
    #[default]
    None,
    /// Border on top edge only.
    Top,
    /// Border on bottom edge only.
    Bottom,
    /// Border on left edge only.
    Left,
    /// Border on right edge only.
    Right,
    /// Border on all edges.
    All,
}

impl BorderLocation {
    /// CSS class suffix for this border location.
    pub fn css_class(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Top => Some("border-top"),
            Self::Bottom => Some("border-bottom"),
            Self::Left => Some("border-left"),
            Self::Right => Some("border-right"),
            Self::All => Some("border-all"),
        }
    }
}

/// Visual style variants for bar buttons.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum BarButtonVariant {
    /// Icon + label, minimal background.
    #[default]
    Basic,
    /// Icon in colored pill container that blends into button edge.
    BlockPrefix,
    /// Button background with colored icon container inside.
    IconSquare,
}

impl BarButtonVariant {
    /// CSS class name for this variant.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::BlockPrefix => "block-prefix",
            Self::IconSquare => "icon-square",
        }
    }
}
