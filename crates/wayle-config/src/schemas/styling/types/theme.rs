use crate::infrastructure::themes::Palette;

/// A discovered theme available for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeEntry {
    /// Color palette for this theme.
    pub palette: Palette,
    /// Whether this is a built-in theme or user-defined.
    pub builtin: bool,
}
