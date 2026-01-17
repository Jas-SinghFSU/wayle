//! Color-related styling types.
//!
//! Palette colors and color values for theming.

use std::{borrow::Cow, fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::validated::HexColor;
use crate::infrastructure::themes::Palette;

/// Semantic color names from the palette.
///
/// Invalid color names fail at config parse time rather than silently falling back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PaletteColor {
    /// Base background color (darkest).
    Bg,
    /// Card and sidebar background.
    Surface,
    /// Raised element background.
    Elevated,
    /// Primary text color.
    Fg,
    /// Secondary/muted text color.
    FgMuted,
    /// Accent color for interactive elements.
    Primary,
    /// Red semantic color.
    Red,
    /// Yellow semantic color.
    Yellow,
    /// Green semantic color.
    Green,
    /// Blue semantic color.
    Blue,
}

impl PaletteColor {
    /// CSS variable reference (e.g., `var(--palette-primary)`).
    pub fn css_var(self) -> &'static str {
        match self {
            Self::Bg => "var(--palette-bg)",
            Self::Surface => "var(--palette-surface)",
            Self::Elevated => "var(--palette-elevated)",
            Self::Fg => "var(--palette-fg)",
            Self::FgMuted => "var(--palette-fg-muted)",
            Self::Primary => "var(--palette-primary)",
            Self::Red => "var(--palette-red)",
            Self::Yellow => "var(--palette-yellow)",
            Self::Green => "var(--palette-green)",
            Self::Blue => "var(--palette-blue)",
        }
    }
}

impl fmt::Display for PaletteColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bg => "bg",
            Self::Surface => "surface",
            Self::Elevated => "elevated",
            Self::Fg => "fg",
            Self::FgMuted => "fg-muted",
            Self::Primary => "primary",
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Blue => "blue",
        };
        write!(f, "{}", s)
    }
}

/// Error when parsing an invalid palette color name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Unknown palette color: '{0}'")]
pub struct InvalidPaletteColor(pub String);

impl FromStr for PaletteColor {
    type Err = InvalidPaletteColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bg" => Ok(Self::Bg),
            "surface" => Ok(Self::Surface),
            "elevated" => Ok(Self::Elevated),
            "fg" => Ok(Self::Fg),
            "fg-muted" => Ok(Self::FgMuted),
            "primary" => Ok(Self::Primary),
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(InvalidPaletteColor(s.to_owned())),
        }
    }
}

/// Palette reference, custom hex color, or transparent.
///
/// Palette references (e.g., `"surface"`) update when themes change.
/// Custom hex values (e.g., `"#414868"`) remain fixed.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorValue {
    /// Palette color reference. Resolves to different hex values per theme.
    Palette(PaletteColor),

    /// Fixed hex color (e.g., `"#414868"`). Ignores theme changes.
    Custom(HexColor),

    /// Fully transparent. Maps to CSS `transparent` keyword.
    Transparent,
}

impl Default for ColorValue {
    fn default() -> Self {
        Self::Palette(PaletteColor::Fg)
    }
}

impl ColorValue {
    /// CSS value for inline styles. Palette returns `var(--*)`, custom returns hex.
    pub fn to_css(&self) -> Cow<'static, str> {
        match self {
            ColorValue::Palette(color) => Cow::Borrowed(color.css_var()),
            ColorValue::Custom(hex) => Cow::Owned(hex.to_string()),
            ColorValue::Transparent => Cow::Borrowed("transparent"),
        }
    }

    /// Resolves to a concrete CSS color string using the given palette.
    pub fn resolve<'a>(&'a self, palette: &'a Palette) -> &'a str {
        match self {
            ColorValue::Palette(color) => palette.get(*color),
            ColorValue::Custom(hex) => hex.as_str(),
            ColorValue::Transparent => "transparent",
        }
    }

    /// Whether this references a palette color.
    pub fn is_palette(&self) -> bool {
        matches!(self, ColorValue::Palette(_))
    }

    /// GUI label (e.g., `"Palette: Surface"` or `"Custom: #414868"`).
    pub fn display_label(&self) -> String {
        match self {
            ColorValue::Palette(color) => format!("Palette: {color}"),
            ColorValue::Custom(hex) => format!("Custom: {hex}"),
            ColorValue::Transparent => "Transparent".to_owned(),
        }
    }
}

impl Serialize for ColorValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ColorValue::Palette(color) => serializer.serialize_str(&color.to_string()),
            ColorValue::Custom(hex) => serializer.serialize_str(hex.as_str()),
            ColorValue::Transparent => serializer.serialize_str("transparent"),
        }
    }
}

impl<'de> Deserialize<'de> for ColorValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "transparent" {
            Ok(ColorValue::Transparent)
        } else if s.starts_with('#') {
            HexColor::new(s)
                .map(ColorValue::Custom)
                .map_err(serde::de::Error::custom)
        } else {
            s.parse::<PaletteColor>()
                .map(ColorValue::Palette)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl schemars::JsonSchema for ColorValue {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ColorValue")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

/// Source of color palette values.
///
/// Dynamic providers (Matugen, Pywal, Wallust) inject palette tokens at runtime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ThemeProvider {
    /// Static theming using Wayle's built-in palettes. User color overrides are respected.
    #[default]
    Wayle,
    /// Dynamic theming via Matugen. Palette tokens are injected at runtime.
    Matugen,
    /// Dynamic theming via Pywal. Palette tokens are injected at runtime.
    Pywal,
    /// Dynamic theming via Wallust. Palette tokens are injected at runtime.
    Wallust,
}

impl fmt::Display for ThemeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Wayle => "wayle",
            Self::Matugen => "matugen",
            Self::Pywal => "pywal",
            Self::Wallust => "wallust",
        };
        write!(f, "{s}")
    }
}
