use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Configuration for the clock's appearance in the status bar.
///
/// Controls visual elements specific to how the clock module appears
/// when displayed in the main status bar.
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct ClockButtonConfig {
    /// Whether to display a clock icon alongside the time text.
    #[default(true)]
    pub show_icon: ConfigProperty<bool>,
}
