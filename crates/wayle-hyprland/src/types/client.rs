use serde::{Deserialize, Deserializer};

use crate::{
    Address, FocusHistoryId, MonitorId, ProcessId, WorkspaceInfo, deserialize_optional_address,
    deserialize_optional_string,
};

/// Window dimensions in pixels.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ClientSize {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

pub(crate) fn deserialize_window_size<'de, D>(deserializer: D) -> Result<ClientSize, D::Error>
where
    D: Deserializer<'de>,
{
    let [width, height]: [u32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(ClientSize { width, height })
}

/// Window position in screen coordinates.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ClientLocation {
    /// X coordinate in pixels.
    pub x: i32,
    /// Y coordinate in pixels.
    pub y: i32,
}

pub(crate) fn deserialize_window_location<'de, D>(
    deserializer: D,
) -> Result<ClientLocation, D::Error>
where
    D: Deserializer<'de>,
{
    let [x, y]: [i32; 2] = Deserialize::deserialize(deserializer)?;

    Ok(ClientLocation { x, y })
}

/// Window fullscreen state.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(from = "u8")]
pub enum FullscreenMode {
    /// Not fullscreen.
    None = 0,
    /// Fullscreen mode.
    Full = 1,
    /// Maximized mode.
    Maximize = 2,
}

impl From<u8> for FullscreenMode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Full,
            2 => Self::Maximize,
            _ => Self::None,
        }
    }
}

/// Client (window) data from hyprctl.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientData {
    pub address: Address,
    pub mapped: bool,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_window_location")]
    pub at: ClientLocation,
    #[serde(deserialize_with = "deserialize_window_size")]
    pub size: ClientSize,
    pub workspace: WorkspaceInfo,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: MonitorId,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: ProcessId,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: FullscreenMode,
    pub fullscreen_client: FullscreenMode,
    pub grouped: Vec<Address>,
    pub tags: Vec<String>,
    #[serde(deserialize_with = "deserialize_optional_address")]
    pub swallowing: Option<Address>,
    pub focus_history_id: FocusHistoryId,
    pub inhibiting_idle: bool,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub xdg_tag: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_string")]
    pub xdg_description: Option<String>,
}
