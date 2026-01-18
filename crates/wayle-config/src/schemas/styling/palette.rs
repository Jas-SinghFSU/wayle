use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use super::HexColor;
use crate::infrastructure::themes::palettes::catppuccin_mocha as mocha;

fn hex(s: &str) -> HexColor {
    HexColor::new(s).unwrap_or_else(|_| HexColor::new(mocha::RED).unwrap_or_default())
}

/// Color palette configuration for the active theme.
#[wayle_config]
pub struct PaletteConfig {
    /// Base background color (darkest).
    #[default(hex(mocha::BG))]
    pub bg: ConfigProperty<HexColor>,

    /// Card and sidebar background.
    #[default(hex(mocha::SURFACE))]
    pub surface: ConfigProperty<HexColor>,

    /// Raised element background.
    #[default(hex(mocha::ELEVATED))]
    pub elevated: ConfigProperty<HexColor>,

    /// Primary text color.
    #[default(hex(mocha::FG))]
    pub fg: ConfigProperty<HexColor>,

    /// Secondary text color.
    #[serde(rename = "fg-muted")]
    #[default(hex(mocha::FG_MUTED))]
    pub fg_muted: ConfigProperty<HexColor>,

    /// Accent color for interactive elements.
    #[default(hex(mocha::PRIMARY))]
    pub primary: ConfigProperty<HexColor>,

    /// Red semantic color.
    #[default(hex(mocha::RED))]
    pub red: ConfigProperty<HexColor>,

    /// Yellow semantic color.
    #[default(hex(mocha::YELLOW))]
    pub yellow: ConfigProperty<HexColor>,

    /// Green semantic color.
    #[default(hex(mocha::GREEN))]
    pub green: ConfigProperty<HexColor>,

    /// Blue semantic color.
    #[default(hex(mocha::BLUE))]
    pub blue: ConfigProperty<HexColor>,
}
