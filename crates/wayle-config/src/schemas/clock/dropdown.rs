use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Clock dropdown panel configuration.
///
/// Controls both behavior and styling for the clock's dropdown menu.
/// Properties use noun-first naming with prefixes for logical grouping
/// (e.g., `calendar-*`, `clock-*` properties).
#[wayle_config]
pub struct ClockDropdownConfig {
    /// Whether to display the calendar widget.
    #[serde(rename = "calendar-show")]
    #[default(true)]
    pub calendar_show: ConfigProperty<bool>,

    /// Clock display color in dropdown (CSS color token).
    #[serde(rename = "clock-color")]
    #[default(String::from("fg"))]
    pub clock_color: ConfigProperty<String>,
}
