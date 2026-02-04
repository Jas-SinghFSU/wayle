use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Idle inhibitor module configuration.
///
/// Prevents screen dimming, lock, and suspend when active.
/// Control via CLI: `wayle idle on/off/duration/remaining/status`
#[wayle_config(bar_button)]
pub struct IdleInhibitConfig {
    /// Duration in minutes when service starts. 0 means indefinite.
    #[serde(rename = "startup-duration")]
    #[default(60)]
    pub startup_duration: ConfigProperty<u32>,

    /// Icon when idle inhibitor is inactive.
    #[serde(rename = "icon-inactive")]
    #[default(String::from("tb-coffee-off-symbolic"))]
    pub icon_inactive: ConfigProperty<String>,

    /// Icon when idle inhibitor is active.
    #[serde(rename = "icon-active")]
    #[default(String::from("tb-coffee-symbolic"))]
    pub icon_active: ConfigProperty<String>,

    /// Label format. Use {state}, {remaining}, {duration} placeholders.
    #[serde(rename = "label-format")]
    #[default(String::from("{state}"))]
    pub label_format: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Green))]
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
    #[default(ColorValue::Token(CssToken::Green))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Green))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(0)]
    pub label_max_length: ConfigProperty<u32>,

    /// Button background color token.
    #[serde(rename = "button-bg-color")]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,

    /// Shell command on left click.
    #[serde(rename = "left-click")]
    #[default(String::from("wayle idle toggle --indefinite"))]
    pub left_click: ConfigProperty<String>,

    /// Shell command on right click.
    #[serde(rename = "right-click")]
    #[default(String::from("wayle idle toggle"))]
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

impl ModuleInfoProvider for IdleInhibitConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("idle-inhibit"),
            icon: String::from(""),
            description: String::from("Prevent screen dimming, lock, and suspend"),
            behavior_configs: vec![(String::from("idle-inhibit"), || {
                schema_for!(IdleInhibitConfig)
            })],
            styling_configs: vec![],
        }
    }
}
