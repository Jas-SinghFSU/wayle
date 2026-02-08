//! Type definitions for bar button components.

use std::fmt::{Debug, Formatter, Result as FmtResult};

use wayle_common::ConfigProperty;
pub use wayle_config::schemas::bar::BarButtonVariant;
use wayle_config::schemas::{
    bar::{BorderLocation, IconPosition},
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
    /// Icon color when Auto and Basic variant.
    pub auto_icon_color: CssToken,
}

impl Debug for BarButtonColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarButtonColors").finish_non_exhaustive()
    }
}

/// Behavioral settings for bar buttons (shared across all variants).
#[derive(Clone)]
pub struct BarButtonBehavior {
    /// Max label characters before truncation. Set to 0 to disable.
    pub label_max_chars: ConfigProperty<u32>,
    /// Show the icon.
    pub show_icon: ConfigProperty<bool>,
    /// Show the label (false = icon-only mode).
    pub show_label: ConfigProperty<bool>,
    /// Show the border.
    pub show_border: ConfigProperty<bool>,
    /// Button visibility.
    pub visible: ConfigProperty<bool>,
}

impl Debug for BarButtonBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("BarButtonBehavior")
            .field("show_label", &self.show_label.get())
            .field("visible", &self.visible.get())
            .finish_non_exhaustive()
    }
}

/// Settings for bar buttons.
#[derive(Debug, Clone)]
pub struct BarSettings {
    /// Button visual variant.
    pub variant: ConfigProperty<BarButtonVariant>,
    /// Theme provider for color resolution.
    pub theme_provider: ConfigProperty<ThemeProvider>,
    /// Border placement.
    pub border_location: ConfigProperty<BorderLocation>,
    /// Border width in pixels.
    pub border_width: ConfigProperty<u8>,
    /// Icon position relative to label.
    pub icon_position: ConfigProperty<IconPosition>,
    /// Vertical orientation.
    pub is_vertical: ConfigProperty<bool>,
    /// Scroll sensitivity multiplier.
    pub scroll_sensitivity: f64,
    /// Monitor connector name (e.g., "DP-1", "HDMI-A-1").
    pub monitor_name: Option<String>,
}

/// CSS class constants for bar button modifiers.
pub struct BarButtonClass;

impl BarButtonClass {
    /// Base class for all bar buttons.
    pub const BASE: &'static str = "bar-button";

    /// Applied when label is hidden.
    pub const ICON_ONLY: &'static str = "icon-only";

    /// Applied when icon is hidden.
    pub const LABEL_ONLY: &'static str = "label-only";

    /// Applied for vertical bar orientation.
    pub const VERTICAL: &'static str = "vertical";

    /// Applied when icon is positioned after the label.
    pub const ICON_END: &'static str = "icon-end";
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
