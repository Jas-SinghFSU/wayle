//! Shared type definitions for bar button components.

/// Visual style variants for bar buttons.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BarButtonVariant {
    /// Icon + label, minimal background.
    #[default]
    Basic,
    /// Icon in colored pill container that blends into button edge.
    BlockPrefix,
    /// Button background with colored icon container inside.
    IconSquare,
}

impl BarButtonVariant {
    /// CSS class name for this variant.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::BlockPrefix => "block-prefix",
            Self::IconSquare => "icon-square",
        }
    }
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

/// Common update messages shared by all bar button variants.
///
/// Used by the compositor to forward updates without variant-specific mapping.
#[derive(Debug, Clone)]
pub enum CommonBarButtonMsg {
    /// Update the icon name.
    SetIcon(String),
    /// Update the label text.
    SetLabel(String),
    /// Update the tooltip.
    SetTooltip(Option<String>),
    /// A config property changed; refresh inline CSS.
    ConfigChanged,
}
