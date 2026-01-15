use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Configuration for the battery status module.
///
/// Controls the display and behavior of battery information in the status bar,
/// including percentage display and low battery warnings.
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct BatteryConfig {
    /// Whether the battery module is displayed in the status bar.
    #[default(true)]
    pub enabled: ConfigProperty<bool>,

    /// Whether to show the battery percentage alongside the icon.
    #[default(true)]
    pub show_percentage: ConfigProperty<bool>,

    /// Battery percentage threshold for triggering a low battery warning.
    #[default(20)]
    pub battery_warning: ConfigProperty<u8>,
}
