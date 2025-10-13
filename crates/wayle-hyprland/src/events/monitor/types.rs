use serde::Deserialize;

use crate::{
    Address, DirectScanoutBlocker, MonitorId, Reserved, SolitaryBlocker, TearingBlocker, Transform,
    WorkspaceInfo, deserialize_direct_scanout_blocker, deserialize_mirror_of,
    deserialize_optional_address, deserialize_reserved, deserialize_solitary_blocker,
    deserialize_tearing_blocker, deserialize_transform,
};

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
