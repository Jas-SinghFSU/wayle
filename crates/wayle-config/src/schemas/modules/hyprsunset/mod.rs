use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Hyprsunset (blue light filter) module configuration.
#[wayle_config(bar_button)]
pub struct HyprsunsetConfig {
    /// Format string for the label.
    ///
    /// ## Placeholders
    ///
    /// - `{status}` - Filter status text (On, Off)
    /// - `{temp}` - Current temperature in Kelvin (shows "--" when disabled)
    /// - `{gamma}` - Current gamma percentage (shows "--" when disabled)
    /// - `{config_temp}` - Configured temperature (always available)
    /// - `{config_gamma}` - Configured gamma (always available)
    ///
    /// ## Examples
    ///
    /// - `"{status}"` - "On"
    /// - `"{temp}K {gamma}%"` - "4500K 80%"
    /// - `"{status} ({temp}K)"` - "On (4500K)"
    #[serde(rename = "format")]
    #[default(String::from("{status}"))]
    pub format: ConfigProperty<String>,

    /// Color temperature in Kelvin when filter is enabled. Range: 1000-20000.
    #[serde(rename = "temperature")]
    #[default(5000)]
    pub temperature: ConfigProperty<u32>,

    /// Display gamma percentage when filter is enabled. Range: 0-200.
    #[serde(rename = "gamma")]
    #[default(100)]
    pub gamma: ConfigProperty<u32>,

    /// Icon when filter is disabled (showing normal daylight colors).
    #[serde(rename = "icon-off")]
    #[default(String::from("ld-sun-symbolic"))]
    pub icon_off: ConfigProperty<String>,

    /// Icon when filter is enabled (showing warm night colors).
    #[serde(rename = "icon-on")]
    #[default(String::from("ld-moon-symbolic"))]
    pub icon_on: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Yellow))]
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
    #[default(ColorValue::Token(CssToken::Yellow))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Yellow))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(0)]
    pub label_max_length: ConfigProperty<u32>,

    /// Button background color token.
    #[serde(rename = "button-bg-color")]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,

    /// Action on left click. `:toggle` for built-in on/off, empty for no
    /// action, or a shell command.
    #[serde(rename = "left-click")]
    #[default(String::from(":toggle"))]
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

impl ModuleInfoProvider for HyprsunsetConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("hyprsunset"),
            icon: String::from(""),
            description: String::from("Control Hyprland's blue light filter (night light)"),
            behavior_configs: vec![(String::from("hyprsunset"), || schema_for!(HyprsunsetConfig))],
            styling_configs: vec![],
        }
    }
}
