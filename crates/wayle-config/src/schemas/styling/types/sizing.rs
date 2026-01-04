//! Size and spacing styling types.
//!
//! Icon sizes, padding, and gap classes for layout control.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Icon size class for CSS-based sizing.
///
/// Maps to CSS classes like `.icon-sm`, `.icon-md`, etc.
/// The actual pixel values are defined in SCSS tokens.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum IconSizeClass {
    /// Small icons (--icon-sm token).
    Sm,
    /// Medium icons (--icon-md token).
    #[default]
    Md,
    /// Large icons (--icon-lg token).
    Lg,
    /// Extra large icons (--icon-xl token).
    Xl,
}

impl IconSizeClass {
    /// CSS class for GTK widget styling (e.g., `icon-md`).
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Sm => "icon-sm",
            Self::Md => "icon-md",
            Self::Lg => "icon-lg",
            Self::Xl => "icon-xl",
        }
    }

    /// CSS variable name (e.g., `--icon-md`).
    pub fn css_var(self) -> &'static str {
        match self {
            Self::Sm => "--icon-sm",
            Self::Md => "--icon-md",
            Self::Lg => "--icon-lg",
            Self::Xl => "--icon-xl",
        }
    }
}

/// Padding size class for CSS-based spacing.
///
/// Maps to CSS classes like `.padding-xs`, `.padding-sm`, etc.
/// The actual spacing values are defined in SCSS tokens.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PaddingClass {
    /// Extra small padding (--space-xs token).
    Xs,
    /// Small padding (--space-sm token).
    Sm,
    /// Medium padding (--space-md token).
    #[default]
    Md,
    /// Large padding (--space-lg token).
    Lg,
    /// Extra large padding (--space-xl token).
    Xl,
}

impl PaddingClass {
    /// Uniform padding class (e.g., `padding-md`).
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Xs => "padding-xs",
            Self::Sm => "padding-sm",
            Self::Md => "padding-md",
            Self::Lg => "padding-lg",
            Self::Xl => "padding-xl",
        }
    }

    /// Horizontal padding class (e.g., `padding-x-md`).
    pub fn css_class_x(self) -> &'static str {
        match self {
            Self::Xs => "padding-x-xs",
            Self::Sm => "padding-x-sm",
            Self::Md => "padding-x-md",
            Self::Lg => "padding-x-lg",
            Self::Xl => "padding-x-xl",
        }
    }

    /// Vertical padding class (e.g., `padding-y-md`).
    pub fn css_class_y(self) -> &'static str {
        match self {
            Self::Xs => "padding-y-xs",
            Self::Sm => "padding-y-sm",
            Self::Md => "padding-y-md",
            Self::Lg => "padding-y-lg",
            Self::Xl => "padding-y-xl",
        }
    }

    /// CSS variable name (e.g., `--space-md`).
    pub fn css_var(self) -> &'static str {
        match self {
            Self::Xs => "--space-xs",
            Self::Sm => "--space-sm",
            Self::Md => "--space-md",
            Self::Lg => "--space-lg",
            Self::Xl => "--space-xl",
        }
    }
}

/// Gap size class for spacing between elements.
///
/// Maps to CSS classes like `.gap-xs`, `.gap-sm`, etc.
/// Used for spacing between icon and label in bar buttons.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GapClass {
    /// Extra small gap (--space-xs token).
    Xs,
    /// Small gap (--space-sm token).
    #[default]
    Sm,
    /// Medium gap (--space-md token).
    Md,
    /// Large gap (--space-lg token).
    Lg,
}

impl GapClass {
    /// CSS class for spacing (e.g., `gap-sm`).
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Xs => "gap-xs",
            Self::Sm => "gap-sm",
            Self::Md => "gap-md",
            Self::Lg => "gap-lg",
        }
    }
}
