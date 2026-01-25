use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Bluetooth module configuration.
#[wayle_config(bar_button)]
pub struct BluetoothConfig {
    /// Icon when Bluetooth is disabled or unavailable.
    #[serde(rename = "disabled-icon")]
    #[default(String::from("ld-bluetooth-off-symbolic"))]
    pub disabled_icon: ConfigProperty<String>,

    /// Icon when Bluetooth is on but no devices connected.
    #[serde(rename = "disconnected-icon")]
    #[default(String::from("ld-bluetooth-symbolic"))]
    pub disconnected_icon: ConfigProperty<String>,

    /// Icon when devices are connected.
    #[serde(rename = "connected-icon")]
    #[default(String::from("ld-bluetooth-connected-symbolic"))]
    pub connected_icon: ConfigProperty<String>,

    /// Icon when scanning for devices.
    #[serde(rename = "searching-icon")]
    #[default(String::from("ld-bluetooth-searching-symbolic"))]
    pub searching_icon: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Blue))]
    pub border_color: ConfigProperty<ColorValue>,

    /// Display module icon.
    #[serde(rename = "icon-show")]
    #[default(true)]
    pub icon_show: ConfigProperty<bool>,

    /// Icon foreground color. Auto selects based on variant for contrast.
    #[serde(rename = "icon-color")]
    #[default(ColorValue::Auto)]
    pub icon_color: ConfigProperty<ColorValue>,

    /// Icon container background color token.
    #[serde(rename = "icon-bg-color")]
    #[default(ColorValue::Token(CssToken::Blue))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display connection label (device name or count).
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Blue))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(15)]
    pub label_max_length: ConfigProperty<u32>,

    /// Button background color token.
    #[serde(rename = "button-bg-color")]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,

    /// Reserved for dropdown. Not user-configurable.
    #[serde(rename = "left-click", skip)]
    #[default(String::default())]
    pub left_click: ConfigProperty<String>,

    /// Shell command on right click.
    #[serde(rename = "right-click")]
    #[default(String::default())]
    pub right_click: ConfigProperty<String>,

    /// Shell command on middle click.
    #[serde(rename = "middle-click")]
    #[default(String::default())]
    pub middle_click: ConfigProperty<String>,

    /// Shell command on scroll up.
    #[serde(rename = "scroll-up")]
    #[default(String::default())]
    pub scroll_up: ConfigProperty<String>,

    /// Shell command on scroll down.
    #[serde(rename = "scroll-down")]
    #[default(String::default())]
    pub scroll_down: ConfigProperty<String>,
}

impl ModuleInfoProvider for BluetoothConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("bluetooth"),
            icon: String::from("󰂯"),
            description: String::from("Bluetooth connection status"),
            behavior_configs: vec![(String::from("bluetooth"), || schema_for!(BluetoothConfig))],
            styling_configs: vec![],
        }
    }
}
