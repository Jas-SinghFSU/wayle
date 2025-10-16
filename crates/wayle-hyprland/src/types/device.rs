use serde::Deserialize;

use crate::Address;

/// Mouse or pointer device configuration from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MouseDevice {
    /// Unique memory address identifying this device.
    pub address: Address,
    /// Human-readable device name.
    pub name: String,
    /// Default pointer acceleration speed from libinput (-1.0 to 1.0).
    pub default_speed: f32,
    /// Scroll wheel multiplier factor.
    pub scroll_factor: f32,
}

/// Keyboard device configuration from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardDevice {
    /// Unique memory address identifying this device.
    pub address: Address,
    /// Human-readable device name.
    pub name: String,
    /// XKB rules file path or identifier.
    pub rules: String,
    /// XKB model name for this keyboard.
    pub model: String,
    /// XKB layout name (e.g., "us", "de", "dvorak").
    pub layout: String,
    /// XKB layout variant (e.g., "colemak", "intl").
    pub variant: String,
    /// XKB options string (e.g., "ctrl:nocaps,compose:ralt").
    pub options: String,
    /// Index of the currently active layout when multiple layouts configured.
    pub active_layout_index: u32,
    /// Name of the currently active keymap.
    pub active_keymap: String,
    /// Whether Caps Lock modifier is currently engaged.
    pub caps_lock: bool,
    /// Whether Num Lock modifier is currently engaged.
    pub num_lock: bool,
    /// Whether this is the primary keyboard device.
    pub main: bool,
}

/// Container for all input device information from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DeviceInfo {
    /// All connected mouse and pointer devices.
    pub mice: Vec<MouseDevice>,
    /// All connected keyboard devices.
    pub keyboards: Vec<KeyboardDevice>,
    /// All connected tablet devices (including pads, tablets, and tools).
    pub tablets: Vec<TabletDevice>,
    /// All connected touchscreen devices.
    pub touch: Vec<TouchDevice>,
    /// All connected switch devices (e.g., laptop lid switches).
    pub switches: Vec<SwitchDevice>,
}

/// Graphics tablet device from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TabletDevice {
    /// Unique memory address identifying this device.
    pub address: Address,
    /// Human-readable device name.
    pub name: String,
}

/// Touchscreen device from Hyprland.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TouchDevice {
    /// Unique memory address identifying this device.
    pub address: Address,
    /// Human-readable device name.
    pub name: String,
}

/// Switch device from Hyprland (e.g., laptop lid switch).
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SwitchDevice {
    /// Unique memory address identifying this device.
    pub address: Address,
    /// Human-readable device name.
    pub name: String,
}
