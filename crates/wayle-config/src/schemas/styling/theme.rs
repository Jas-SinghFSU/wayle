use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wayle_common::{ConfigProperty, Property};
use wayle_derive::{ApplyConfigLayer, ApplyRuntimeLayer, ExtractRuntimeValues, SubscribeChanges};

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
///
/// Used when preset is set to "custom".
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
pub struct CustomPalette {
    /// Base background color (darkest).
    pub bg: ConfigProperty<String>,

    /// Card and sidebar background.
    pub surface: ConfigProperty<String>,

    /// Raised element background.
    pub elevated: ConfigProperty<String>,

    /// Primary text color.
    pub fg: ConfigProperty<String>,

    /// Secondary text color.
    #[serde(rename = "fg-muted")]
    pub fg_muted: ConfigProperty<String>,

    /// Accent color for interactive elements.
    pub primary: ConfigProperty<String>,

    /// Red palette color.
    pub red: ConfigProperty<String>,

    /// Yellow palette color.
    pub yellow: ConfigProperty<String>,

    /// Green palette color.
    pub green: ConfigProperty<String>,

    /// Blue palette color.
    pub blue: ConfigProperty<String>,
}

impl Default for CustomPalette {
    fn default() -> Self {
        Self {
            bg: ConfigProperty::new(String::from(defaults::BG)),
            surface: ConfigProperty::new(String::from(defaults::SURFACE)),
            elevated: ConfigProperty::new(String::from(defaults::ELEVATED)),
            fg: ConfigProperty::new(String::from(defaults::FG)),
            fg_muted: ConfigProperty::new(String::from(defaults::FG_MUTED)),
            primary: ConfigProperty::new(String::from(defaults::PRIMARY)),
            red: ConfigProperty::new(String::from(defaults::RED)),
            yellow: ConfigProperty::new(String::from(defaults::YELLOW)),
            green: ConfigProperty::new(String::from(defaults::GREEN)),
            blue: ConfigProperty::new(String::from(defaults::BLUE)),
        }
    }
}

impl CustomPalette {
    /// Convert to a Palette for CSS generation.
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

/// Theme configuration for the Wayle shell.
///
/// Controls the color palette used throughout the interface.
/// Select a preset theme or use "custom" with user-defined colors.
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
pub struct ThemeConfig {
    /// The active color palette
    pub palette: ConfigProperty<Palette>,

    /// Available themes discovered at runtime.
    ///
    /// Populated from built-in themes and user themes in ~/.config/wayle/themes/.
    #[serde(skip)]
    #[schemars(skip)]
    #[wayle(skip)]
    pub available: Property<Vec<ThemeEntry>>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            palette: ConfigProperty::new(catppuccin()),
            available: Property::new(Vec::new()),
        }
    }
}
