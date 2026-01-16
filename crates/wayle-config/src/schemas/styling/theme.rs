use wayle_common::ConfigProperty;
use wayle_derive::wayle_config;

use crate::infrastructure::themes::{Palette, palettes::catppuccin};

mod defaults {
    pub const BG: &str = "#11111b";
    pub const SURFACE: &str = "#181825";
    pub const ELEVATED: &str = "#1e1e2e";
    pub const FG: &str = "#cdd6f4";
    pub const FG_MUTED: &str = "#bac2de";
    pub const PRIMARY: &str = "#b4befe";
    pub const RED: &str = "#f38ba8";
    pub const YELLOW: &str = "#f9e2af";
    pub const GREEN: &str = "#a6e3a1";
    pub const BLUE: &str = "#74c7ec";
}

/// A discovered theme available for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeEntry {
    /// Color palette for this theme.
    pub palette: Palette,
    /// Whether this is a built-in theme or user-defined.
    pub builtin: bool,
}

/// User-defined custom color palette.
#[wayle_config]
pub struct CustomPalette {
    /// Base background color (darkest).
    #[default(String::from(defaults::BG))]
    pub bg: ConfigProperty<String>,

    /// Card and sidebar background.
    #[default(String::from(defaults::SURFACE))]
    pub surface: ConfigProperty<String>,

    /// Raised element background.
    #[default(String::from(defaults::ELEVATED))]
    pub elevated: ConfigProperty<String>,

    /// Primary text color.
    #[default(String::from(defaults::FG))]
    pub fg: ConfigProperty<String>,

    /// Secondary text color.
    #[serde(rename = "fg-muted")]
    #[default(String::from(defaults::FG_MUTED))]
    pub fg_muted: ConfigProperty<String>,

    /// Accent color for interactive elements.
    #[default(String::from(defaults::PRIMARY))]
    pub primary: ConfigProperty<String>,

    /// Red palette color.
    #[default(String::from(defaults::RED))]
    pub red: ConfigProperty<String>,

    /// Yellow palette color.
    #[default(String::from(defaults::YELLOW))]
    pub yellow: ConfigProperty<String>,

    /// Green palette color.
    #[default(String::from(defaults::GREEN))]
    pub green: ConfigProperty<String>,

    /// Blue palette color.
    #[default(String::from(defaults::BLUE))]
    pub blue: ConfigProperty<String>,
}

impl CustomPalette {
    /// Converts to a `Palette` for CSS generation.
    pub fn to_palette(&self) -> Palette {
        Palette {
            name: String::from("custom"),
            bg: self.bg.get(),
            surface: self.surface.get(),
            elevated: self.elevated.get(),
            fg: self.fg.get(),
            fg_muted: self.fg_muted.get(),
            primary: self.primary.get(),
            red: self.red.get(),
            yellow: self.yellow.get(),
            green: self.green.get(),
            blue: self.blue.get(),
        }
    }
}

/// Color palette configuration. Select a preset or use custom colors.
#[wayle_config]
pub struct ThemeConfig {
    /// The active color palette.
    #[default(catppuccin())]
    pub active: ConfigProperty<Palette>,

    /// Available themes discovered at runtime.
    ///
    /// Populated from built-in themes and user themes in ~/.config/wayle/themes/.
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    #[default(Vec::new())]
    pub available: ConfigProperty<Vec<ThemeEntry>>,
}
