use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::docs::{ModuleInfo, ModuleInfoProvider};

/// Clock module configuration.
#[wayle_config]
pub struct ClockConfig {
    /// Time format string using strftime syntax.
    #[default(String::from("%H:%M"))]
    pub format: ConfigProperty<String>,

    /// Display clock icon in bar.
    #[serde(rename = "bar-icon-show")]
    #[default(true)]
    pub bar_icon_show: ConfigProperty<bool>,

    /// Bar icon color (CSS color token).
    #[serde(rename = "bar-icon-color")]
    #[default(String::from("primary"))]
    pub bar_icon_color: ConfigProperty<String>,

    /// Display calendar widget in dropdown.
    #[serde(rename = "dropdown-calendar-show")]
    #[default(true)]
    pub dropdown_calendar_show: ConfigProperty<bool>,

    /// Clock display color in dropdown (CSS color token).
    #[serde(rename = "dropdown-clock-color")]
    #[default(String::from("fg"))]
    pub dropdown_clock_color: ConfigProperty<String>,
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
