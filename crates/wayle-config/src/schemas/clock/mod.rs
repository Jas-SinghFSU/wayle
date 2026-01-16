mod bar;
mod dropdown;

pub use bar::ClockBarConfig;
pub use dropdown::ClockDropdownConfig;
use schemars::schema_for;
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::docs::{ModuleInfo, ModuleInfoProvider};

/// Clock module configuration for bar and dropdown calendar display.
#[wayle_config]
pub struct ClockConfig {
    /// Time format string using strftime syntax.
    #[default(String::from("%H:%M"))]
    pub format: ConfigProperty<String>,

    /// Bar button configuration.
    #[serde(default)]
    pub bar: ClockBarConfig,

    /// Dropdown panel configuration.
    #[serde(default)]
    pub dropdown: ClockDropdownConfig,
}

impl ModuleInfoProvider for ClockConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("clock"),
            icon: String::from("󰥔"),
            description: String::from("Clock display and calendar settings"),
            behavior_configs: vec![
                (String::from("bar"), || schema_for!(ClockBarConfig)),
                (String::from("dropdown"), || {
                    schema_for!(ClockDropdownConfig)
                }),
            ],
            styling_configs: vec![],
        }
    }
}
