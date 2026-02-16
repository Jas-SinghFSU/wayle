use schemars::schema_for;
use wayle_common::{ConfigProperty, process::ClickAction};
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Notification module configuration.
#[wayle_config(bar_button)]
pub struct NotificationConfig {
    /// Icon shown when no notifications and DND is off.
    #[serde(rename = "icon-name")]
    #[default(String::from("ld-bell-symbolic"))]
    pub icon_name: ConfigProperty<String>,

    /// Icon shown when notifications exist.
    #[serde(rename = "icon-unread")]
    #[default(String::from("ld-bell-dot-symbolic"))]
    pub icon_unread: ConfigProperty<String>,

    /// Icon shown when Do Not Disturb is active.
    #[serde(rename = "icon-dnd")]
    #[default(String::from("ld-bell-off-symbolic"))]
    pub icon_dnd: ConfigProperty<String>,

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

    /// Display notification count label.
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

    /// Action on left click.
    #[serde(rename = "left-click")]
    #[default(ClickAction::None)]
    pub left_click: ConfigProperty<ClickAction>,

    /// Action on right click. Default toggles Do Not Disturb.
    #[serde(rename = "right-click")]
    #[default(ClickAction::Shell(String::from("wayle notify dnd")))]
    pub right_click: ConfigProperty<ClickAction>,

    /// Action on middle click.
    #[serde(rename = "middle-click")]
    #[default(ClickAction::None)]
    pub middle_click: ConfigProperty<ClickAction>,

    /// Action on scroll up.
    #[serde(rename = "scroll-up")]
    #[default(ClickAction::None)]
    pub scroll_up: ConfigProperty<ClickAction>,

    /// Action on scroll down.
    #[serde(rename = "scroll-down")]
    #[default(ClickAction::None)]
    pub scroll_down: ConfigProperty<ClickAction>,
}

impl ModuleInfoProvider for NotificationConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("notification"),
            icon: String::from("󰂚"),
            description: String::from("Notification management"),
            behavior_configs: vec![(String::from("notification"), || {
                schema_for!(NotificationConfig)
            })],
            styling_configs: vec![],
        }
    }
}
