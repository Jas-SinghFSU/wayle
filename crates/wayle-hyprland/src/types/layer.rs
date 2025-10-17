use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::Deserialize;

use crate::{Address, ProcessId};

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

impl From<&str> for LayerLevel {
    fn from(value: &str) -> Self {
        match value {
            "0" => Self::Background,
            "1" => Self::Bottom,
            "2" => Self::Top,
            "3" => Self::Overlay,
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

/// Layer surface data from hyprctl.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LayerData {
    pub address: Address,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub namespace: String,
    pub monitor: String,
    pub level: LayerLevel,
    pub pid: ProcessId,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MonitorLayers {
    pub levels: HashMap<String, Vec<LayerResponse>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LayerResponse {
    pub address: Address,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub namespace: String,
    pub pid: ProcessId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_level_from_u8_converts_background() {
        let level = LayerLevel::from(0u8);

        assert_eq!(level, LayerLevel::Background);
    }

    #[test]
    fn layer_level_from_u8_converts_bottom() {
        let level = LayerLevel::from(1u8);

        assert_eq!(level, LayerLevel::Bottom);
    }

    #[test]
    fn layer_level_from_u8_converts_top() {
        let level = LayerLevel::from(2u8);

        assert_eq!(level, LayerLevel::Top);
    }

    #[test]
    fn layer_level_from_u8_converts_overlay() {
        let level = LayerLevel::from(3u8);

        assert_eq!(level, LayerLevel::Overlay);
    }

    #[test]
    fn layer_level_from_u8_returns_unknown_for_invalid() {
        let level = LayerLevel::from(99u8);

        assert_eq!(level, LayerLevel::Unknown);
    }

    #[test]
    fn layer_level_from_str_converts_all_valid_values() {
        assert_eq!(LayerLevel::from("0"), LayerLevel::Background);
        assert_eq!(LayerLevel::from("1"), LayerLevel::Bottom);
        assert_eq!(LayerLevel::from("2"), LayerLevel::Top);
        assert_eq!(LayerLevel::from("3"), LayerLevel::Overlay);
    }

    #[test]
    fn layer_level_from_str_returns_unknown_for_invalid() {
        let level = LayerLevel::from("invalid");

        assert_eq!(level, LayerLevel::Unknown);
    }
}
