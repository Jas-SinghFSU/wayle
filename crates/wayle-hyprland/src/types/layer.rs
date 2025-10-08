use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum LayerLevel {
    Background = 0,
    Bottom = 1,
    Top = 2,
    Overlay = 3,
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
