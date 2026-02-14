mod palette;
mod types;

pub use palette::PaletteConfig;
pub use types::{
    ColorValue, CssToken, FontWeightClass, GapClass, HexColor, IconSizeClass, InvalidCssToken,
    InvalidHexColor, NormalizedF64, PaddingClass, Percentage, RadiusClass, RoundingLevel,
    ScaleFactor, Spacing, TextSizeClass, ThemeEntry, ThemeProvider,
};
use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::infrastructure::themes::Palette;

/// Styling configuration. Changes trigger stylesheet recompilation.
#[wayle_config]
pub struct StylingConfig {
    /// Scale multiplier for dropdowns, popovers, and dialogs.
    #[default(ScaleFactor::new(1.0))]
    pub scale: ConfigProperty<ScaleFactor>,

    /// Corner rounding for dropdowns, popovers, and dialogs.
    #[default(RoundingLevel::default())]
    pub rounding: ConfigProperty<RoundingLevel>,

    /// Theme provider (wayle, matugen, pywal, wallust).
    #[serde(rename = "theme-provider")]
    #[default(ThemeProvider::default())]
    pub theme_provider: ConfigProperty<ThemeProvider>,

    /// Monitor whose wallpaper drives color extraction. Empty uses the first available.
    #[serde(rename = "theming-monitor")]
    #[default(String::new())]
    pub theming_monitor: ConfigProperty<String>,

    /// Active color palette.
    pub palette: PaletteConfig,

    /// Discovered themes (runtime-populated).
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(Vec::new())]
    pub available: ConfigProperty<Vec<ThemeEntry>>,
}

impl StylingConfig {
    /// Assembles a palette from the individual color fields.
    pub fn palette(&self) -> Palette {
        Palette {
            bg: self.palette.bg.get().to_string(),
            surface: self.palette.surface.get().to_string(),
            elevated: self.palette.elevated.get().to_string(),
            fg: self.palette.fg.get().to_string(),
            fg_muted: self.palette.fg_muted.get().to_string(),
            primary: self.palette.primary.get().to_string(),
            red: self.palette.red.get().to_string(),
            yellow: self.palette.yellow.get().to_string(),
            green: self.palette.green.get().to_string(),
            blue: self.palette.blue.get().to_string(),
        }
    }
}
