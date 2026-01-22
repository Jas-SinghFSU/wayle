use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Clock module configuration.
#[wayle_config]
pub struct ClockConfig {
    /// Time format string using strftime syntax.
    #[default(String::from("%H:%M"))]
    pub format: ConfigProperty<String>,

    /// Tooltip text shown on hover.
    #[default(None)]
    pub tooltip: ConfigProperty<Option<String>>,

    /// Symbolic icon name.
    #[serde(rename = "icon-name")]
    #[default(String::from("tb-calendar-time-symbolic"))]
    pub icon_name: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::BorderAccent))]
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
    #[default(ColorValue::Token(CssToken::Accent))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display text label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Accent))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Truncate label with ellipsis.
    #[serde(rename = "label-truncate")]
    #[default(false)]
    pub label_truncate: ConfigProperty<bool>,

    /// Max characters before truncation.
    #[serde(rename = "label-max-length")]
    #[default(20)]
    pub label_max_length: ConfigProperty<u32>,

    /// Button background color token.
    #[serde(rename = "button-bg-color")]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,

    /// Shell command on left click.
    #[serde(rename = "left-click")]
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

impl ModuleInfoProvider for ClockConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("clock"),
            icon: String::from("󰥔"),
            description: String::from("Clock display and calendar settings"),
            behavior_configs: vec![(String::from("clock"), || schema_for!(ClockConfig))],
            styling_configs: vec![],
        }
    }
}
