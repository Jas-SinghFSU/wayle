use std::{convert::Infallible, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::{Address, MonitorId, WorkspaceInfo, deserialize_optional_address};

/// Reserved screen space on a monitor for panels, bars, etc.
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Reserved {
    /// Reserved space at the top edge in pixels.
    pub top: u32,
    /// Reserved space at the bottom edge in pixels.
    pub bottom: u32,
    /// Reserved space at the left edge in pixels.
    pub left: u32,
    /// Reserved space at the right edge in pixels.
    pub right: u32,
}

pub(crate) fn deserialize_reserved<'de, D>(deserializer: D) -> Result<Reserved, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: [u32; 4] = Deserialize::deserialize(deserializer)?;
    Ok(Reserved {
        top: arr[0],
        bottom: arr[1],
        left: arr[2],
        right: arr[3],
    })
}

/// Monitor transform (rotation and flip).
///
/// Based on the Wayland `wl_output_transform` protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    /// No transform.
    Normal = 0,
    /// Rotated 90 degrees counter-clockwise.
    Transform90 = 1,
    /// Rotated 180 degrees counter-clockwise.
    Transform180 = 2,
    /// Rotated 270 degrees counter-clockwise.
    Transform270 = 3,
    /// Flipped 180 degrees around a vertical axis.
    Flipped = 4,
    /// Flipped and rotated 90 degrees counter-clockwise.
    Flipped90 = 5,
    /// Flipped and rotated 180 degrees counter-clockwise.
    Flipped180 = 6,
    /// Flipped and rotated 270 degrees counter-clockwise.
    Flipped270 = 7,
}

pub(crate) fn deserialize_transform<'de, D>(deserializer: D) -> Result<Transform, D::Error>
where
    D: Deserializer<'de>,
{
    let value: u8 = Deserialize::deserialize(deserializer)?;
    Ok(match value {
        0 => Transform::Normal,
        1 => Transform::Transform90,
        2 => Transform::Transform180,
        3 => Transform::Transform270,
        4 => Transform::Flipped,
        5 => Transform::Flipped90,
        6 => Transform::Flipped180,
        7 => Transform::Flipped270,
        _ => Transform::Normal,
    })
}

/// Reasons why solitary mode (tearing optimization) is blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolitaryBlocker {
    /// Unknown reason or new blocker not yet supported.
    Unknown,
    /// Notification is present.
    Notification,
    /// Session lock is active.
    Lock,
    /// Invalid workspace configuration.
    Workspace,
    /// Monitor is in windowed mode.
    Windowed,
    /// Drag and drop is active.
    Dnd,
    /// Special workspace is active.
    Special,
    /// Window has alpha channel.
    Alpha,
    /// Workspace has offset.
    Offset,
    /// No suitable candidate window.
    Candidate,
    /// Window is not opaque.
    Opaque,
    /// Surface has transformations.
    Transform,
    /// Other overlays are present.
    Overlays,
    /// Floating windows are present.
    Float,
    /// Multiple workspaces are visible.
    Workspaces,
    /// Window has subsurfaces.
    Surfaces,
    /// Configuration error.
    Errorbar,
}

impl FromStr for SolitaryBlocker {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "NOTIFICATION" => Self::Notification,
            "LOCK" => Self::Lock,
            "WORKSPACE" => Self::Workspace,
            "WINDOWED" => Self::Windowed,
            "DND" => Self::Dnd,
            "SPECIAL" => Self::Special,
            "ALPHA" => Self::Alpha,
            "OFFSET" => Self::Offset,
            "CANDIDATE" => Self::Candidate,
            "OPAQUE" => Self::Opaque,
            "TRANSFORM" => Self::Transform,
            "OVERLAYS" => Self::Overlays,
            "FLOAT" => Self::Float,
            "WORKSPACES" => Self::Workspaces,
            "SURFACES" => Self::Surfaces,
            "ERRORBAR" => Self::Errorbar,
            _ => Self::Unknown,
        })
    }
}

pub(crate) fn deserialize_solitary_blocker<'de, D>(
    deserializer: D,
) -> Result<Vec<SolitaryBlocker>, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(arr
        .iter()
        .map(|s| s.parse().unwrap_or(SolitaryBlocker::Unknown))
        .collect())
}

/// Reasons why tearing is blocked for a monitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TearingBlocker {
    /// Unknown reason or new blocker not yet supported.
    Unknown,
    /// Next frame is not torn.
    NotTorn,
    /// User settings disable tearing.
    User,
    /// Zoom is active.
    Zoom,
    /// Monitor does not support tearing.
    Support,
    /// No suitable candidate window.
    Candidate,
    /// Window settings prevent tearing.
    Window,
}

impl FromStr for TearingBlocker {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "NOT_TORN" => Self::NotTorn,
            "USER" => Self::User,
            "ZOOM" => Self::Zoom,
            "SUPPORT" => Self::Support,
            "CANDIDATE" => Self::Candidate,
            "WINDOW" => Self::Window,
            _ => Self::Unknown,
        })
    }
}

pub(crate) fn deserialize_tearing_blocker<'de, D>(
    deserializer: D,
) -> Result<Vec<TearingBlocker>, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(arr
        .iter()
        .map(|s| s.parse().unwrap_or(TearingBlocker::Unknown))
        .collect())
}

/// Reasons why direct scanout is blocked for a monitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectScanoutBlocker {
    /// Unknown reason or new blocker not yet supported.
    Unknown,
    /// User settings disable direct scanout.
    User,
    /// Monitor is in windowed mode.
    Windowed,
    /// Content type is incompatible.
    Content,
    /// Monitor is being mirrored.
    Mirror,
    /// Screen recording or screenshot is active.
    Record,
    /// Software rendering or cursors are active.
    Sw,
    /// No suitable candidate window.
    Candidate,
    /// Invalid surface.
    Surface,
    /// Surface has transformations.
    Transform,
    /// Invalid DMA buffer.
    Dma,
    /// Tearing is active.
    Tearing,
    /// Activation failed.
    Failed,
    /// Color management is active.
    Cm,
}

impl FromStr for DirectScanoutBlocker {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "USER" => Self::User,
            "WINDOWED" => Self::Windowed,
            "CONTENT" => Self::Content,
            "MIRROR" => Self::Mirror,
            "RECORD" => Self::Record,
            "SW" => Self::Sw,
            "CANDIDATE" => Self::Candidate,
            "SURFACE" => Self::Surface,
            "TRANSFORM" => Self::Transform,
            "DMA" => Self::Dma,
            "TEARING" => Self::Tearing,
            "FAILED" => Self::Failed,
            "CM" => Self::Cm,
            _ => Self::Unknown,
        })
    }
}

pub(crate) fn deserialize_direct_scanout_blocker<'de, D>(
    deserializer: D,
) -> Result<Vec<DirectScanoutBlocker>, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(arr
        .iter()
        .map(|s| s.parse().unwrap_or(DirectScanoutBlocker::Unknown))
        .collect())
}

pub(crate) fn deserialize_mirror_of<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s == "none" || s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MonitorData {
    pub id: MonitorId,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: u32,
    pub height: u32,
    pub physical_width: u32,
    pub physical_height: u32,
    pub refresh_rate: f32,
    pub x: i32,
    pub y: i32,
    pub active_workspace: WorkspaceInfo,
    pub special_workspace: WorkspaceInfo,
    #[serde(deserialize_with = "deserialize_reserved")]
    pub reserved: Reserved,
    pub scale: f32,
    #[serde(deserialize_with = "deserialize_transform")]
    pub transform: Transform,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool,
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub solitary: Option<Address>,
    #[serde(deserialize_with = "deserialize_solitary_blocker")]
    pub solitary_blocked_by: Vec<SolitaryBlocker>,
    pub actively_tearing: bool,
    #[serde(deserialize_with = "deserialize_tearing_blocker")]
    pub tearing_blocked_by: Vec<TearingBlocker>,
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub direct_scanout_to: Option<Address>,
    #[serde(deserialize_with = "deserialize_direct_scanout_blocker")]
    pub direct_scanout_blocked_by: Vec<DirectScanoutBlocker>,
    pub disabled: bool,
    pub current_format: String,
    #[serde(deserialize_with = "deserialize_mirror_of")]
    pub mirror_of: Option<String>,
    pub available_modes: Vec<String>,
}
