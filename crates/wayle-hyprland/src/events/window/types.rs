use serde::Deserialize;

use crate::{
    Address, FocusHistoryId, FullscreenMode, MonitorId, ProcessId, WindowLocation, WindowSize,
    WorkspaceInfo, deserialize_optional_address, deserialize_optional_string,
    deserialize_window_location, deserialize_window_size,
};

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WindowData {
    pub address: Address,
    pub mapped: bool,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_window_location")]
    pub at: WindowLocation,
    #[serde(deserialize_with = "deserialize_window_size")]
    pub size: WindowSize,
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
