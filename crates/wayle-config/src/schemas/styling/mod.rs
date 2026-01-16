mod theme;
mod types;

pub use theme::ThemeConfig;
pub use types::{
    ColorValue, FontWeightClass, GapClass, IconSizeClass, InvalidPaletteColor, PaddingClass,
    PaletteColor, RadiusClass, RoundingLevel, TextSizeClass, ThemeEntry, ThemeProvider,
};
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

/// Global styling configuration. Changes trigger stylesheet recompilation.
#[wayle_config]
pub struct StylingConfig {
    /// Determines which provider handles theming.
    #[serde(rename = "theme-provider")]
    #[default(ThemeProvider::default())]
    pub theme_provider: ConfigProperty<ThemeProvider>,

    /// Color palette configuration.
    pub theme: ThemeConfig,

    /// Global UI scale multiplier affecting all rem-based sizing.
    #[default(1.0)]
    pub scale: ConfigProperty<f32>,

    /// Global rounding preference (none, sm, md, lg).
    #[default(RoundingLevel::default())]
    pub rounding: ConfigProperty<RoundingLevel>,
}
