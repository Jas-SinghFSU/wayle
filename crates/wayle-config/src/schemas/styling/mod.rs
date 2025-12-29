mod components;
mod fonts;
mod theme;
mod types;

pub use components::{ButtonStyling, DropdownStyling};
pub use fonts::FontConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use theme::{CustomPalette, ThemeConfig, ThemeEntry};
pub use types::{ColorValue, InvalidPaletteColor, PaletteColor, RoundingLevel};
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
    /// Color palette configuration.
    pub theme: ThemeConfig,

    /// Font family configuration.
    pub fonts: FontConfig,

    /// Global UI scale multiplier affecting all rem-based sizing.
    pub scale: ConfigProperty<f32>,

    /// Global rounding preference (none, sm, md, lg).
    pub rounding: ConfigProperty<RoundingLevel>,

    /// Bar background
    #[serde(rename = "bar-bg")]
    pub bar_bg: ConfigProperty<ColorValue>,
}

impl Default for StylingConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            fonts: FontConfig::default(),
            scale: ConfigProperty::new(1.0),
            rounding: ConfigProperty::new(RoundingLevel::default()),
            bar_bg: ConfigProperty::new(ColorValue::Palette(PaletteColor::Primary)),
        }
    }
}
