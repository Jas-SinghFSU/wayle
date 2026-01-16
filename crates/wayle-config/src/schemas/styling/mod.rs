mod components;
mod fonts;
mod theme;
mod types;

pub use components::{BasicButtonSizing, BlockPrefixSizing, IconSquareSizing};
pub use fonts::FontConfig;
pub use theme::{CustomPalette, ThemeConfig, ThemeEntry};
pub use types::{
    ColorValue, FontWeightClass, GapClass, IconSizeClass, InvalidPaletteColor, PaddingClass,
    PaletteColor, RadiusClass, RoundingLevel, TextSizeClass, ThemeProvider,
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

    /// Font family configuration.
    pub fonts: FontConfig,

    /// Global UI scale multiplier affecting all rem-based sizing.
    #[default(1.0)]
    pub scale: ConfigProperty<f32>,

    /// Bar-specific scale multiplier for bar and bar button sizing.
    ///
    /// Allows the bar to be scaled independently from dropdown content.
    /// Users often want a compact bar but readable dropdowns.
    #[serde(rename = "bar-scale")]
    #[default(1.0)]
    pub bar_scale: ConfigProperty<f32>,

    /// Global rounding preference (none, sm, md, lg).
    #[default(RoundingLevel::default())]
    pub rounding: ConfigProperty<RoundingLevel>,

    /// Bar background color.
    #[serde(rename = "bar-bg")]
    #[default(ColorValue::Palette(PaletteColor::Primary))]
    pub bar_bg: ConfigProperty<ColorValue>,

    /// Sizing for Basic bar button variant.
    #[serde(rename = "bar-button-basic")]
    pub bar_button_basic: BasicButtonSizing,

    /// Sizing for BlockPrefix bar button variant.
    #[serde(rename = "bar-button-block-prefix")]
    pub bar_button_block_prefix: BlockPrefixSizing,

    /// Sizing for IconSquare bar button variant.
    #[serde(rename = "bar-button-icon-square")]
    pub bar_button_icon_square: IconSquareSizing,
}
