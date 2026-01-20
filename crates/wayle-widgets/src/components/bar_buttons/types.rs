//! Shared type definitions for bar button components.

use std::fmt::{Debug, Formatter, Result as FmtResult};

use wayle_common::ConfigProperty;
pub use wayle_config::schemas::bar::BarButtonVariant;
use wayle_config::schemas::{
    bar::BorderLocation,
    styling::{ColorValue, CssToken, ThemeProvider},
};

/// Colors for bar buttons (shared across all variants).
///
/// Not all variants use every color slot - they take what applies to their structure.
#[derive(Clone)]
pub struct BarButtonColors {
    /// Icon symbolic color.
    pub icon_color: ConfigProperty<ColorValue>,
    /// Label text color.
    pub label_color: ConfigProperty<ColorValue>,
    /// Icon container background (BlockPrefix/IconSquare only).
    pub icon_background: ConfigProperty<ColorValue>,
    /// Button background.
    pub button_background: ConfigProperty<ColorValue>,
    /// Border color.
    pub border_color: ConfigProperty<ColorValue>,
}

impl Default for BarButtonColors {
    fn default() -> Self {
        Self {
            icon_color: ConfigProperty::new(ColorValue::Token(CssToken::Accent)),
            label_color: ConfigProperty::new(ColorValue::Token(CssToken::Accent)),
            icon_background: ConfigProperty::new(ColorValue::Token(CssToken::Accent)),
            button_background: ConfigProperty::new(ColorValue::Token(CssToken::BgSurfaceElevated)),
            border_color: ConfigProperty::new(ColorValue::Token(CssToken::BorderAccent)),
        }
    }
}

impl Debug for BarButtonColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarButtonColors").finish_non_exhaustive()
    }
}

/// Behavioral settings for bar buttons (shared across all variants).
#[derive(Clone)]
pub struct BarButtonBehavior {
    /// Truncate label with ellipsis.
    pub truncation_enabled: ConfigProperty<bool>,
    /// Max characters before truncation.
    pub truncation_size: ConfigProperty<u32>,
    /// Show the label (false = icon-only mode).
    pub show_label: ConfigProperty<bool>,
    /// Button visibility.
    pub visible: ConfigProperty<bool>,
    /// Vertical orientation for vertical bars.
    pub vertical: ConfigProperty<bool>,
}

impl Default for BarButtonBehavior {
    fn default() -> Self {
        Self {
            truncation_enabled: ConfigProperty::new(false),
            truncation_size: ConfigProperty::new(20),
            show_label: ConfigProperty::new(true),
            visible: ConfigProperty::new(true),
            vertical: ConfigProperty::new(false),
        }
    }
}

impl Debug for BarButtonBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarButtonBehavior")
            .field("show_label", &self.show_label.get())
            .field("visible", &self.visible.get())
            .field("vertical", &self.vertical.get())
            .finish_non_exhaustive()
    }
}

/// Complete configuration for a bar button module.
///
/// Does not implement `Default` - modules must explicitly provide bar-wide
/// settings from the global config to ensure consistency across all buttons.
#[derive(Debug, Clone)]
pub struct BarButtonConfig {
    /// Initial variant to display.
    pub variant: ConfigProperty<BarButtonVariant>,
    /// Color settings (per-module).
    pub colors: BarButtonColors,
    /// Behavioral settings (per-module).
    pub behavior: BarButtonBehavior,
    /// Theme provider for color resolution (bar-wide).
    pub theme_provider: ConfigProperty<ThemeProvider>,
    /// Border placement (bar-wide).
    pub border_location: ConfigProperty<BorderLocation>,
    /// Border width in pixels (bar-wide).
    pub border_width: ConfigProperty<u8>,
}

/// CSS class constants for bar button modifiers.
pub struct BarButtonClass;

impl BarButtonClass {
    /// Base class for all bar buttons.
    pub const BASE: &'static str = "bar-button";

    /// Applied when label is hidden.
    pub const ICON_ONLY: &'static str = "icon-only";

    /// Applied for vertical bar orientation.
    pub const VERTICAL: &'static str = "vertical";
}

/// Output events emitted by bar buttons.
///
/// Parent components handle these events to perform actions like
/// opening menus, adjusting values, or toggling states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarButtonOutput {
    /// Left mouse button.
    LeftClick,
    /// Right mouse button.
    RightClick,
    /// Middle mouse button.
    MiddleClick,
    /// Scroll wheel up.
    ScrollUp,
    /// Scroll wheel down.
    ScrollDown,
}
