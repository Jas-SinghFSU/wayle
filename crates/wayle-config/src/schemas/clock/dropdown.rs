use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Configuration for the clock's dropdown menu.
///
/// Controls the content and behavior of the dropdown that appears
/// when clicking on the clock module.
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct ClockDropdownConfig {
    /// Whether to display a calendar widget in the dropdown menu.
    #[default(true)]
    pub show_calendar: ConfigProperty<bool>,
}
