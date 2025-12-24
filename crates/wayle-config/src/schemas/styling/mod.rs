mod components;
mod fonts;
mod theme;
mod types;

pub use components::{ButtonStyling, DropdownStyling};
pub use fonts::FontConfig;
pub use theme::{CustomPalette, ThemeConfig, ThemeEntry};
pub use types::{ColorValue, InvalidPaletteColor, PaletteColor};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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

    /// Bar background
    pub bar_bg: ConfigProperty<ColorValue>,
}

impl Default for StylingConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            fonts: FontConfig::default(),
            scale: ConfigProperty::new(1.0),
            bar_bg: ConfigProperty::new(ColorValue::Palette(PaletteColor::Primary)),
        }
    }
}
