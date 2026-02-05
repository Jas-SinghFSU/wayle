mod icons;

use std::collections::HashMap;

use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

pub use self::icons::BUILTIN_MAPPINGS;
use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Window title module configuration.
#[wayle_config(bar_button)]
pub struct WindowTitleConfig {
    /// Format string for the label.
    ///
    /// ## Placeholders
    ///
    /// - `{title}` - Window title
    /// - `{app}` - Application name (WM_CLASS on Hyprland)
    ///
    /// ## Examples
    ///
    /// - `"{title}"` - "README.md - VSCode"
    /// - `"{app}: {title}"` - "firefox: GitHub"
    #[default(String::from("{title}"))]
    pub format: ConfigProperty<String>,

    /// Fallback icon when no mapping matches.
    #[serde(rename = "icon-name")]
    #[default(String::from("ld-app-window-symbolic"))]
    pub icon_name: ConfigProperty<String>,

    /// Icon mappings. Glob patterns to icon names.
    ///
    /// Patterns match window class by default. Prefix with `title:` to match
    /// window title instead. User mappings are checked before built-in mappings.
    #[serde(rename = "icon-mappings")]
    #[default(HashMap::new())]
    pub icon_mappings: ConfigProperty<HashMap<String, String>>,

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

    /// Icon foreground color.
    #[serde(rename = "icon-color")]
    #[default(ColorValue::Auto)]
    pub icon_color: ConfigProperty<ColorValue>,

    /// Icon container background color token.
    #[serde(rename = "icon-bg-color")]
    #[default(ColorValue::Token(CssToken::Blue))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display text label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Blue))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(50)]
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

impl ModuleInfoProvider for WindowTitleConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("window-title"),
            icon: String::from("󱂬"),
            description: String::from("Active window title display"),
            behavior_configs: vec![(String::from("window-title"), || {
                schema_for!(WindowTitleConfig)
            })],
            styling_configs: vec![],
        }
    }
}
