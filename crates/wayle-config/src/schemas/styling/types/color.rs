//! Color-related styling types.
//!
//! Palette colors and color values for theming.

use std::{borrow::Cow, fmt, str::FromStr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::infrastructure::themes::Palette;

/// Semantic color names from the palette.
///
/// These map to the 10 palette colors that drive the visual theme.
/// Using an enum ensures compile-time validation and catches invalid
/// color names at config parse time rather than silently falling back.
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
    /// Returns the CSS variable reference for this palette color.
    ///
    /// Used when generating inline CSS that references theme colors.
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

/// A color value that can reference a palette color or be a custom hex.
///
/// When the user selects a color from their palette (e.g., "surface", "primary"),
/// it stays linked to the theme and updates when themes change. Custom hex values
/// remain fixed regardless of theme.
///
/// # Serialization
///
/// Serializes to a plain string. The `#` prefix distinguishes custom values:
/// - `"surface"` -> Palette
/// - `"#414868"` -> Custom
#[derive(Debug, Clone, PartialEq)]
pub enum ColorValue {
    /// References a color from the user's palette.
    ///
    /// Must be resolved against a [`Palette`] to get the actual hex value.
    /// Changes when the user switches themes.
    Palette(PaletteColor),

    /// A fixed hex color value (e.g., "#414868").
    ///
    /// Ignores theme changes. Used when the user wants a specific color
    /// that doesn't follow the palette.
    Custom(String),
}

impl Default for ColorValue {
    fn default() -> Self {
        Self::Palette(PaletteColor::Fg)
    }
}

impl ColorValue {
    /// Returns the CSS value for inline style generation.
    ///
    /// Palette colors return CSS variable references (e.g., `var(--primary)`).
    /// Custom colors return the hex value directly.
    pub fn to_css(&self) -> Cow<'static, str> {
        match self {
            ColorValue::Palette(color) => Cow::Borrowed(color.css_var()),
            ColorValue::Custom(hex) => Cow::Owned(hex.clone()),
        }
    }

    /// Resolves the color value to a hex string using the given palette.
    ///
    /// Palette references are looked up by name. Custom values pass through unchanged.
    pub fn resolve<'a>(&'a self, palette: &'a Palette) -> &'a str {
        match self {
            ColorValue::Palette(color) => palette.get(*color),
            ColorValue::Custom(hex) => hex,
        }
    }

    /// Returns true if this color references a palette color.
    pub fn is_palette(&self) -> bool {
        matches!(self, ColorValue::Palette(_))
    }

    /// Returns a display label for the GUI (e.g., "Palette: Surface" or "Custom: #414868").
    pub fn display_label(&self) -> String {
        match self {
            ColorValue::Palette(color) => format!("Palette: {}", color),
            ColorValue::Custom(hex) => format!("Custom: {}", hex),
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
            ColorValue::Custom(hex) => serializer.serialize_str(hex),
        }
    }
}

impl<'de> Deserialize<'de> for ColorValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('#') {
            Ok(ColorValue::Custom(s))
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

/// Determines the source of color palette values.
///
/// When set to a dynamic provider (Matugen, Pywal, Wallust), user-configured
/// custom colors are ignored and palette tokens from the provider are used instead.
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
