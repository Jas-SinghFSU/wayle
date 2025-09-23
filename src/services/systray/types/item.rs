use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

/// Raw icon pixmap data from D-Bus.
/// (width, height, argb32_data)
pub type RawPixmap = (i32, i32, Vec<u8>);

/// Collection of raw icon pixmaps from D-Bus.
pub type RawPixmaps = Vec<RawPixmap>;

/// Raw tooltip data from D-Bus.
/// (icon_name, icon_pixmaps, title, description)
pub type RawTooltip = (String, RawPixmaps, String, String);

/// Describes the category of a StatusNotifierItem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    /// The item describes the status of a generic application.
    #[serde(rename = "application_status")]
    ApplicationStatus,
    /// The item describes the status of communication oriented applications.
    #[serde(rename = "communications")]
    Communications,
    /// The item describes services of the system not seen as a stand alone application.
    #[serde(rename = "system_services")]
    SystemServices,
    /// The item describes the state and control of a particular hardware.
    #[serde(rename = "hardware")]
    Hardware,
}

impl Default for Category {
    fn default() -> Self {
        Self::ApplicationStatus
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "ApplicationStatus" => Self::ApplicationStatus,
            "Communications" => Self::Communications,
            "SystemServices" => Self::SystemServices,
            "Hardware" => Self::Hardware,
            _ => Self::ApplicationStatus,
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::ApplicationStatus => write!(f, "ApplicationStatus"),
            Self::Communications => write!(f, "Communications"),
            Self::SystemServices => write!(f, "SystemServices"),
            Self::Hardware => write!(f, "Hardware"),
        }
    }
}

/// Describes the status of a StatusNotifierItem or its associated application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    /// The item doesn't convey important information to the user.
    #[serde(rename = "passive")]
    Passive,
    /// The item is active and should be shown to the user.
    #[serde(rename = "active")]
    Active,
    /// The item carries really important information for the user.
    #[serde(rename = "needs_attention")]
    NeedsAttention,
}

impl Default for Status {
    fn default() -> Self {
        Self::Passive
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "Passive" => Self::Passive,
            "Active" => Self::Active,
            "NeedsAttention" => Self::NeedsAttention,
            _ => Self::Passive,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Passive => write!(f, "Passive"),
            Self::Active => write!(f, "Active"),
            Self::NeedsAttention => write!(f, "NeedsAttention"),
        }
    }
}

/// Icon pixmap data for a StatusNotifierItem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IconPixmap {
    /// Width of the icon in pixels.
    pub width: i32,
    /// Height of the icon in pixels.
    pub height: i32,
    /// ARGB32 binary data in network byte order.
    pub data: Vec<u8>,
}

impl From<(i32, i32, Vec<u8>)> for IconPixmap {
    fn from(tuple: (i32, i32, Vec<u8>)) -> Self {
        Self {
            width: tuple.0,
            height: tuple.1,
            data: tuple.2,
        }
    }
}

impl From<IconPixmap> for (i32, i32, Vec<u8>) {
    fn from(pixmap: IconPixmap) -> Self {
        (pixmap.width, pixmap.height, pixmap.data)
    }
}

/// Tooltip information for a StatusNotifierItem.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Tooltip {
    /// Freedesktop-compliant name for an icon.
    pub icon_name: String,
    /// Icon data as pixmaps.
    pub icon_pixmap: Vec<IconPixmap>,
    /// Title for this tooltip.
    pub title: String,
    /// Descriptive text for this tooltip (may contain HTML markup).
    pub description: String,
}

impl From<(String, Vec<(i32, i32, Vec<u8>)>, String, String)> for Tooltip {
    fn from(tuple: (String, Vec<(i32, i32, Vec<u8>)>, String, String)) -> Self {
        Self {
            icon_name: tuple.0,
            icon_pixmap: tuple.1.into_iter().map(IconPixmap::from).collect(),
            title: tuple.2,
            description: tuple.3,
        }
    }
}

/// Scroll orientation for StatusNotifierItem scroll events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollOrientation {
    /// Horizontal scroll.
    #[serde(rename = "horizontal")]
    Horizontal,
    /// Vertical scroll.
    #[serde(rename = "vertical")]
    Vertical,
}

impl From<&str> for ScrollOrientation {
    fn from(s: &str) -> Self {
        match s {
            "horizontal" => Self::Horizontal,
            "vertical" => Self::Vertical,
            _ => Self::Vertical,
        }
    }
}

impl Display for ScrollOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}
