use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schemas::styling::PaletteColor;

/// Built-in theme palettes.
pub mod palettes;
/// Theme discovery utilities
pub mod utils;

/// Theme palette for CSS generation.
///
/// Contains the 10 palette colors that drive the entire visual theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Palette {
    /// Theme identifier.
    pub name: String,
    /// Base background color (darkest).
    pub bg: String,
    /// Card and sidebar background.
    pub surface: String,
    /// Raised element background.
    pub elevated: String,
    /// Primary text color.
    pub fg: String,
    /// Secondary text color.
    pub fg_muted: String,
    /// Accent color for interactive elements.
    pub primary: String,
    /// Red palette color.
    pub red: String,
    /// Yellow palette color.
    pub yellow: String,
    /// Green palette color.
    pub green: String,
    /// Blue palette color.
    pub blue: String,
}

impl Palette {
    /// Looks up a color by its semantic name.
    ///
    /// Returns a reference to the hex color string for the given palette color.
    pub fn get(&self, color: PaletteColor) -> &str {
        match color {
            PaletteColor::Bg => &self.bg,
            PaletteColor::Surface => &self.surface,
            PaletteColor::Elevated => &self.elevated,
            PaletteColor::Fg => &self.fg,
            PaletteColor::FgMuted => &self.fg_muted,
            PaletteColor::Primary => &self.primary,
            PaletteColor::Red => &self.red,
            PaletteColor::Yellow => &self.yellow,
            PaletteColor::Green => &self.green,
            PaletteColor::Blue => &self.blue,
        }
    }
}
