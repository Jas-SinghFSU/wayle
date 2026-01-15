use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Media module configuration.
///
/// Controls media player display in the status bar. Properties use noun-first
/// naming with prefixes for logical grouping.
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
