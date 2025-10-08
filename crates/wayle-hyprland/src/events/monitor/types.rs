use serde::Deserialize;

use crate::{
    Address, DirectScanoutBlocker, MonitorId, Reserved, SolitaryBlocker, TearingBlocker, Transform,
    WorkspaceInfo, deserialize_direct_scanout_blocker, deserialize_mirror_of,
    deserialize_optional_address, deserialize_reserved, deserialize_solitary_blocker,
    deserialize_tearing_blocker, deserialize_transform,
};

/// Monitor information from Hyprland.
///
/// Represents the complete state of a connected display including geometry,
/// capabilities, active workspaces, and rendering optimization blockers.
/// Obtained from `hyprctl monitors -j` or monitor-related events.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MonitorData {
    /// Hyprland's internal monitor ID.
    pub id: MonitorId,
    /// Monitor connector name (e.g., "DP-1", "HDMI-A-1").
    pub name: String,
    /// Human-readable monitor description from EDID.
    pub description: String,
    /// Monitor manufacturer from EDID.
    pub make: String,
    /// Monitor model from EDID.
    pub model: String,
    /// Monitor serial number from EDID.
    pub serial: String,
    /// Current resolution width in pixels.
    pub width: u32,
    /// Current resolution height in pixels.
    pub height: u32,
    /// Physical display width in millimeters from EDID.
    pub physical_width: u32,
    /// Physical display height in millimeters from EDID.
    pub physical_height: u32,
    /// Current refresh rate in Hertz.
    pub refresh_rate: f32,
    /// Monitor X position in compositor layout (left edge).
    pub x: i32,
    /// Monitor Y position in compositor layout (top edge).
    pub y: i32,
    /// Currently active regular workspace on this monitor.
    pub active_workspace: WorkspaceInfo,
    /// Currently active special (scratchpad) workspace on this monitor.
    pub special_workspace: WorkspaceInfo,
    /// Screen space reserved for layer surfaces (panels, bars, etc).
    #[serde(deserialize_with = "deserialize_reserved")]
    pub reserved: Reserved,
    /// Display scale factor for HiDPI (1.0 = 100%, 1.5 = 150%, etc).
    pub scale: f32,
    /// Physical rotation and flip transformation applied to display output.
    #[serde(deserialize_with = "deserialize_transform")]
    pub transform: Transform,
    /// Whether this monitor currently has keyboard focus.
    pub focused: bool,
    /// Display Power Management Signaling status (true = on, false = off).
    pub dpms_status: bool,
    /// Variable Refresh Rate enabled (requires monitor support).
    pub vrr: bool,
    /// Address of the sole fullscreen window when solitary mode active.
    ///
    /// Contains window address as hex string when a single window is fullscreen
    /// and eligible for tearing optimizations. None when multiple windows visible
    /// or solitary mode blocked.
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub solitary: Option<Address>,
    /// Reasons preventing solitary mode (tearing optimization).
    ///
    /// Solitary mode allows a single fullscreen window to bypass the compositor
    /// for lower latency. Multiple blockers can be active simultaneously.
    #[serde(deserialize_with = "deserialize_solitary_blocker")]
    pub solitary_blocked_by: Vec<SolitaryBlocker>,
    /// Whether tearing is currently active for this monitor.
    pub actively_tearing: bool,
    /// Reasons preventing tearing mode.
    ///
    /// Tearing mode allows lower latency rendering at the cost of visual artifacts.
    /// Multiple blockers can be active simultaneously.
    #[serde(deserialize_with = "deserialize_tearing_blocker")]
    pub tearing_blocked_by: Vec<TearingBlocker>,
    /// Address of window using direct scanout if active.
    ///
    /// Direct scanout bypasses compositing for a single fullscreen window,
    /// improving performance. Contains window address as hex string when active.
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub direct_scanout_to: Option<Address>,
    /// Reasons preventing direct scanout.
    ///
    /// Direct scanout allows a window to display directly without compositing.
    /// Multiple blockers can be active simultaneously.
    #[serde(deserialize_with = "deserialize_direct_scanout_blocker")]
    pub direct_scanout_blocked_by: Vec<DirectScanoutBlocker>,
    /// Whether monitor is administratively disabled in configuration.
    pub disabled: bool,
    /// Current DRM pixel format (e.g., "XRGB8888", "XRGB2101010").
    pub current_format: String,
    /// Monitor ID this display is mirroring (cloning).
    ///
    /// Contains the name of the source monitor when mirroring is active.
    /// None when operating as independent display.
    #[serde(deserialize_with = "deserialize_mirror_of")]
    pub mirror_of: Option<String>,
    /// Available video modes reported by monitor.
    ///
    /// Each string contains resolution, refresh rate, and other mode details
    /// as reported by the display hardware.
    pub available_modes: Vec<String>,
}
