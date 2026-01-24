use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::{
    docs::{ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// Volume module configuration.
#[wayle_config(bar_button)]
pub struct VolumeConfig {
    /// Icons for volume levels from low to maximum.
    ///
    /// The percentage is divided evenly among icons. With 3 icons:
    /// 1-33% uses icons[0], 34-66% uses icons[1], 67-100% uses icons[2].
    #[serde(rename = "level-icons")]
    #[default(vec![
        String::from("ld-volume-symbolic"),
        String::from("ld-volume-1-symbolic"),
        String::from("ld-volume-2-symbolic"),
    ])]
    pub level_icons: ConfigProperty<Vec<String>>,

    /// Icon shown when audio output is muted.
    #[serde(rename = "muted-icon")]
    #[default(String::from("ld-volume-x-symbolic"))]
    pub muted_icon: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Red))]
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
    #[default(ColorValue::Token(CssToken::Red))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display percentage label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Red))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(0)]
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

    /// Shell command on middle click. Default toggles mute.
    #[serde(rename = "middle-click")]
    #[default(String::from("wayle audio mute"))]
    pub middle_click: ConfigProperty<String>,

    /// Shell command on scroll up. Default increases volume.
    #[serde(rename = "scroll-up")]
    #[default(String::from("wayle audio volume +5"))]
    pub scroll_up: ConfigProperty<String>,

    /// Shell command on scroll down. Default decreases volume.
    #[serde(rename = "scroll-down")]
    #[default(String::from("wayle audio volume -5"))]
    pub scroll_down: ConfigProperty<String>,
}

impl ModuleInfoProvider for VolumeConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("volume"),
            icon: String::from("󰕾"),
            description: String::from("Audio volume control and mute toggle"),
            behavior_configs: vec![(String::from("volume"), || schema_for!(VolumeConfig))],
            styling_configs: vec![],
        }
    }
}
