use wayle_common::Property;

use crate::{
    Address, ClientLocation, ClientSize, FocusHistoryId, FullscreenMode, MonitorId, ProcessId,
    WorkspaceInfo,
};

pub struct Window {
    pub address: Property<Address>,
    pub mapped: Property<bool>,
    pub hidden: Property<bool>,
    pub at: Property<ClientLocation>,
    pub size: Property<ClientSize>,
    pub workspace: Property<WorkspaceInfo>,
    pub floating: Property<bool>,
    pub pseudo: Property<bool>,
    pub monitor: Property<MonitorId>,
    pub class: Property<String>,
    pub title: Property<String>,
    pub initial_class: Property<String>,
    pub initial_title: Property<String>,
    pub pid: Property<ProcessId>,
    pub xwayland: Property<bool>,
    pub pinned: Property<bool>,
    pub fullscreen: Property<FullscreenMode>,
    pub fullscreen_client: Property<FullscreenMode>,
    pub grouped: Property<Vec<Address>>,
    pub tags: Property<Vec<String>>,
    pub swallowing: Property<Option<Address>>,
    pub focus_history_id: Property<FocusHistoryId>,
    pub inhibiting_idle: Property<bool>,
    pub xdg_tag: Property<Option<String>>,
    pub xdg_description: Property<Option<String>>,
}
