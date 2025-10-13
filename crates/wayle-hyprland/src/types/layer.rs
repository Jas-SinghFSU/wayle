use std::fmt::{self, Display};

/// Layer namespace identifier.
pub type Namespace = String;

/// Layer surface level in the compositor stack.
#[derive(Debug, Clone, PartialEq)]
pub enum LayerLevel {
    /// Background layer (lowest).
    Background = 0,
    /// Bottom layer.
    Bottom = 1,
    /// Top layer.
    Top = 2,
    /// Overlay layer (highest).
    Overlay = 3,
    /// Unknown layer level.
    Unknown = 4,
}

impl From<u8> for LayerLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Background,
            1 => Self::Bottom,
            2 => Self::Top,
            3 => Self::Overlay,
            _ => Self::Unknown,
        }
    }
}

impl Display for LayerLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Background => write!(f, "background"),
            Self::Bottom => write!(f, "bottom"),
            Self::Top => write!(f, "top"),
            Self::Overlay => write!(f, "overlay"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}
