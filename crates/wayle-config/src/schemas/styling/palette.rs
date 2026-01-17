use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use super::HexColor;

fn hex(s: &str) -> HexColor {
    HexColor::new(s).expect("builtin hex color")
}

/// Color palette configuration for the active theme.
#[wayle_config]
pub struct PaletteConfig {
    /// Base background color (darkest).
    #[default(hex("#11111b"))]
    pub bg: ConfigProperty<HexColor>,

    /// Card and sidebar background.
    #[default(hex("#181825"))]
    pub surface: ConfigProperty<HexColor>,

    /// Raised element background.
    #[default(hex("#1e1e2e"))]
    pub elevated: ConfigProperty<HexColor>,

    /// Primary text color.
    #[default(hex("#cdd6f4"))]
    pub fg: ConfigProperty<HexColor>,

    /// Secondary text color.
    #[serde(rename = "fg-muted")]
    #[default(hex("#bac2de"))]
    pub fg_muted: ConfigProperty<HexColor>,

    /// Accent color for interactive elements.
    #[default(hex("#b4befe"))]
    pub primary: ConfigProperty<HexColor>,

    /// Red semantic color.
    #[default(hex("#f38ba8"))]
    pub red: ConfigProperty<HexColor>,

    /// Yellow semantic color.
    #[default(hex("#f9e2af"))]
    pub yellow: ConfigProperty<HexColor>,

    /// Green semantic color.
    #[default(hex("#a6e3a1"))]
    pub green: ConfigProperty<HexColor>,

    /// Blue semantic color.
    #[default(hex("#74c7ec"))]
    pub blue: ConfigProperty<HexColor>,
}
