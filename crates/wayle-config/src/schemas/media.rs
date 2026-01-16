use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Media player module configuration for status bar display.
#[wayle_config]
pub struct MediaConfig {
    /// Whether the media module is displayed.
    #[default(true)]
    pub enabled: ConfigProperty<bool>,

    /// Player bus name patterns to exclude from discovery.
    #[serde(rename = "players-ignored")]
    #[default(Vec::new())]
    pub players_ignored: ConfigProperty<Vec<String>>,
}
