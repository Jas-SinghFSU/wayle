use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Core clock functionality settings.
///
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct ClockGeneralConfig {
    /// Time format string using strftime syntax.
    #[default(String::from("%H:%M"))]
    pub format: ConfigProperty<String>,
}
