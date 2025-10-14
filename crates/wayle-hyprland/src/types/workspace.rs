use serde::Deserialize;

use crate::{Address, MonitorId, WorkspaceId, deserialize_optional_address};

/// Workspace data from hyprctl.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkspaceData {
    pub id: WorkspaceId,
    pub name: String,
    pub monitor: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: MonitorId,
    pub windows: u16,
    #[serde(rename = "hasfullscreen")]
    pub fullscreen: bool,
    #[serde(
        rename = "lastwindow",
        deserialize_with = "deserialize_optional_address"
    )]
    pub last_window: Option<Address>,
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
    #[serde(rename = "ispersistent")]
    pub persistent: bool,
}
