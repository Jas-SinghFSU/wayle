use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Font family configuration.
#[wayle_config]
pub struct FontConfig {
    /// Sans-serif font family for UI text and labels.
    #[default(String::from("Inter"))]
    pub sans: ConfigProperty<String>,

    /// Monospace font family for code and technical content.
    #[default(String::from("JetBrains Mono"))]
    pub mono: ConfigProperty<String>,
}
