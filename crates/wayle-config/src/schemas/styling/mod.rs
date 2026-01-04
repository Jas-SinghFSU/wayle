mod components;
mod fonts;
mod theme;
mod types;

pub use components::{
    BasicButtonSizing, BlockPrefixSizing, ButtonStyling, DropdownStyling, IconSquareSizing,
};
pub use fonts::FontConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use theme::{CustomPalette, ThemeConfig, ThemeEntry};
pub use types::{
    ColorValue, FontWeightClass, GapClass, IconSizeClass, InvalidPaletteColor, PaddingClass,
    PaletteColor, RadiusClass, RoundingLevel, TextSizeClass, ThemeProvider,
};
use wayle_common::ConfigProperty;
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

/// Global styling configuration for the Wayle shell.
///
/// Groups all CSS-affecting properties. Changes to any field trigger
/// stylesheet recompilation.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    ApplyConfigLayer,
    ApplyRuntimeLayer,
    ExtractRuntimeValues,
    SubscribeChanges,
)]
#[serde(default)]
pub struct StylingConfig {
    /// Determine who styles the wayle ecosystem
    #[serde(rename = "theme-provider")]
    pub theme_provider: ConfigProperty<ThemeProvider>,

    /// Color palette configuration.
    pub theme: ThemeConfig,

    /// Font family configuration.
    pub fonts: FontConfig,

    /// Global UI scale multiplier affecting all rem-based sizing.
    pub scale: ConfigProperty<f32>,

    /// Bar-specific scale multiplier for bar and bar button sizing.
    ///
    /// Allows the bar to be scaled independently from dropdown content.
    /// Users often want a compact bar but readable dropdowns.
    #[serde(rename = "bar-scale")]
    pub bar_scale: ConfigProperty<f32>,

    /// Global rounding preference (none, sm, md, lg).
    pub rounding: ConfigProperty<RoundingLevel>,

    /// Bar background
    #[serde(rename = "bar-bg")]
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

impl Default for StylingConfig {
    fn default() -> Self {
        Self {
            theme_provider: ConfigProperty::new(ThemeProvider::default()),
            theme: ThemeConfig::default(),
            fonts: FontConfig::default(),
            scale: ConfigProperty::new(1.0),
            bar_scale: ConfigProperty::new(1.0),
            rounding: ConfigProperty::new(RoundingLevel::default()),
            bar_bg: ConfigProperty::new(ColorValue::Palette(PaletteColor::Primary)),
            bar_button_basic: BasicButtonSizing::default(),
            bar_button_block_prefix: BlockPrefixSizing::default(),
            bar_button_icon_square: IconSquareSizing::default(),
        }
    }
}
