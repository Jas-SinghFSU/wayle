//! Border radius and rounding styling types.
//!
//! Global rounding preferences and per-component radius overrides.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// CSS variable references for semantic rounding tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundingCssValues {
    /// Rounding for interactive elements (buttons, inputs, chips).
    pub element: &'static str,
    /// Rounding for containers (cards, dialogs, dropdowns).
    pub container: &'static str,
}

/// Global rounding preference for UI components.
///
/// Controls how rounded corners appear throughout the shell. A single setting
/// applies proportionally to both interactive elements and containers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum RoundingLevel {
    /// Sharp corners (no rounding).
    None,
    /// Subtle rounding.
    Sm,
    /// Moderate rounding (default).
    #[default]
    Md,
    /// Pronounced rounding.
    Lg,
}

impl RoundingLevel {
    /// Returns global CSS variable references for element and container rounding.
    ///
    /// Elements get the base level, containers get one step larger for
    /// perceptual consistency on larger surfaces.
    pub fn to_css_values(self) -> RoundingCssValues {
        match self {
            Self::None => RoundingCssValues {
                element: "var(--radius-none)",
                container: "var(--radius-none)",
            },
            Self::Sm => RoundingCssValues {
                element: "var(--radius-sm)",
                container: "var(--radius-md)",
            },
            Self::Md => RoundingCssValues {
                element: "var(--radius-md)",
                container: "var(--radius-lg)",
            },
            Self::Lg => RoundingCssValues {
                element: "var(--radius-lg)",
                container: "var(--radius-xl)",
            },
        }
    }

    /// Returns bar-specific CSS variable references for element and container rounding.
    ///
    /// Uses `--bar-radius-*` tokens which scale with `$bar-scale` independently.
    pub fn to_bar_css_values(self) -> RoundingCssValues {
        match self {
            Self::None => RoundingCssValues {
                element: "var(--radius-none)",
                container: "var(--radius-none)",
            },
            Self::Sm => RoundingCssValues {
                element: "var(--bar-radius-sm)",
                container: "var(--bar-radius-md)",
            },
            Self::Md => RoundingCssValues {
                element: "var(--bar-radius-md)",
                container: "var(--bar-radius-lg)",
            },
            Self::Lg => RoundingCssValues {
                element: "var(--bar-radius-lg)",
                container: "var(--bar-radius-xl)",
            },
        }
    }
}

/// Border radius class for component rounding.
///
/// Maps directly to the raw `--radius-*` tokens for component-level
/// rounding control. Provides individual component overrides independent
/// of the global `RoundingLevel` preference.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum RadiusClass {
    /// No rounding (--radius-none).
    None,
    /// Small rounding (--radius-sm).
    Sm,
    /// Medium rounding (--radius-md).
    #[default]
    Md,
    /// Large rounding (--radius-lg).
    Lg,
    /// Extra large rounding (--radius-xl).
    Xl,
    /// Full rounding for pill shapes (--radius-full).
    Full,
}

impl RadiusClass {
    /// CSS class for border radius (e.g., `radius-md`).
    pub fn css_class(self) -> &'static str {
        match self {
            Self::None => "radius-none",
            Self::Sm => "radius-sm",
            Self::Md => "radius-md",
            Self::Lg => "radius-lg",
            Self::Xl => "radius-xl",
            Self::Full => "radius-full",
        }
    }

    /// CSS variable name (e.g., `--radius-md`).
    pub fn css_var(self) -> &'static str {
        match self {
            Self::None => "--radius-none",
            Self::Sm => "--radius-sm",
            Self::Md => "--radius-md",
            Self::Lg => "--radius-lg",
            Self::Xl => "--radius-xl",
            Self::Full => "--radius-full",
        }
    }
}
