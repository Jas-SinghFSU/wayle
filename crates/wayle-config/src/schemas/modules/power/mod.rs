use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Power menu module configuration.
#[wayle_config]
pub struct PowerConfig {
    /// Icon name to display.
    #[serde(rename = "icon-name")]
    #[default(String::from("ld-power-symbolic"))]
    pub icon_name: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Red))]
    pub border_color: ConfigProperty<ColorValue>,

    /// Icon foreground color. Auto selects based on variant for contrast.
    #[serde(rename = "icon-color")]
    #[default(ColorValue::Auto)]
    pub icon_color: ConfigProperty<ColorValue>,

    /// Icon container background color token.
    #[serde(rename = "icon-bg-color")]
    #[default(ColorValue::Token(CssToken::Red))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

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

    /// Hidden: Left click spawns dropdown.
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(String::default())]
    pub left_click: ConfigProperty<String>,

    /// Hidden: icon always shown.
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(true)]
    pub icon_show: ConfigProperty<bool>,

    /// Hidden: label visibility (always false).
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(false)]
    pub label_show: ConfigProperty<bool>,

    /// Hidden: label color (unused).
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(ColorValue::Token(CssToken::Red))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Hidden: label max length (unused).
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(0)]
    pub label_max_length: ConfigProperty<u32>,

    /// Hidden: button background (unused).
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,
}

impl ModuleInfoProvider for PowerConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("power"),
            icon: String::from(""),
            description: String::from("Power menu with shutdown, reboot, and logout options"),
            behavior_configs: vec![(String::from("power"), || schema_for!(PowerConfig))],
            styling_configs: vec![],
        }
    }
}
