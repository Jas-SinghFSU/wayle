use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Styling configuration for the clock module.
///
/// Controls the visual appearance of the clock in both the status bar
/// and dropdown views, including colors, fonts, and icons.
#[wayle_config]
pub struct ClockStyling {
    /// Styling options for the clock button in the bar.
    pub button: ClockButtonStyling,

    /// Styling options for the clock dropdown panel.
    pub dropdown: ClockDropdownStyling,
}

/// Styling configuration for the clock button in the status bar.
///
/// Defines visual properties specific to how the clock appears when
/// displayed as a button in the main status bar.
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct ClockButtonStyling {
    /// CSS color of the clock icon in the bar button.
    #[default(String::from("red"))]
    pub icon: ConfigProperty<String>,
}

/// Styling configuration for the clock dropdown view.
///
/// Controls the visual appearance of the clock when displayed in the
/// dropdown panel, including calendar and time display styling.
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct ClockDropdownStyling {
    /// CSS color of the clock display in the dropdown panel.
    #[default(String::from("red"))]
    pub clock: ConfigProperty<String>,
}
