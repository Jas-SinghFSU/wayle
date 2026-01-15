use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Media service configuration.
///
/// Each field is reactive and can be watched for changes.
#[wayle_config]
pub struct MediaConfig {
    /// List of player bus name patterns to ignore during discovery
    #[default(Vec::new())]
    pub ignored_players: ConfigProperty<Vec<String>>,

    /// Whether the media module is displayed in the status bar.
    #[default(true)]
    pub enabled: ConfigProperty<bool>,
}
