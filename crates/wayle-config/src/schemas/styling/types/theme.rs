use crate::infrastructure::themes::Palette;

/// A discovered theme available for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeEntry {
    /// Theme identifier.
    pub name: String,
    /// Color palette.
    pub palette: Palette,
    /// Built-in or user-defined.
    pub builtin: bool,
}
