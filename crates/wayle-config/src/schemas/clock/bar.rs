use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Clock bar button configuration.
///
/// Controls both behavior and styling for the clock's appearance
/// in the status bar. Properties use noun-first naming with prefixes
/// for logical grouping (e.g., `icon-*` properties).
#[wayle_config]
pub struct ClockBarConfig {
    /// Whether to display the clock icon.
    #[serde(rename = "icon-show")]
    #[default(true)]
    pub icon_show: ConfigProperty<bool>,

    /// Icon color (CSS color token).
    #[serde(rename = "icon-color")]
    #[default(String::from("primary"))]
    pub icon_color: ConfigProperty<String>,
}
