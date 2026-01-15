use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Battery module configuration.
///
/// Controls battery display in the status bar. Properties use noun-first
/// naming with prefixes for logical grouping.
#[wayle_config]
pub struct BatteryConfig {
    /// Whether the battery module is displayed.
    #[default(true)]
    pub enabled: ConfigProperty<bool>,

    /// Whether to show the percentage label alongside the icon.
    #[serde(rename = "percentage-show")]
    #[default(true)]
    pub percentage_show: ConfigProperty<bool>,

    /// Percentage threshold for low battery warning.
    #[serde(rename = "warning-threshold")]
    #[default(20)]
    pub warning_threshold: ConfigProperty<u8>,
}
