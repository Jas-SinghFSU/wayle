use serde::Deserialize;

use crate::Address;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MouseDevice {
    pub address: Address,
    pub name: String,
    pub default_speed: f32,
    pub scroll_factor: f32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardDevice {
    pub address: Address,
    pub name: String,
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: String,
    pub active_layout_index: u32,
    pub active_keymap: String,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub main: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DeviceInfo {
    pub mice: Vec<MouseDevice>,
    pub keyboards: Vec<KeyboardDevice>,
    pub tablets: Vec<TabletDevice>,
    pub touch: Vec<TouchDevice>,
    pub switches: Vec<SwitchDevice>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TabletDevice {
    pub address: Address,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TouchDevice {
    pub address: Address,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SwitchDevice {
    pub address: Address,
    pub name: String,
}
