use wayle_common::Property;

use crate::{Address, MonitorId, WorkspaceId};

pub struct Workspace {
    pub id: Property<WorkspaceId>,
    pub name: Property<String>,
    pub monitor: Property<String>,
    pub monitor_id: Property<MonitorId>,
    pub windows: Property<u16>,
    pub fullscreen: Property<bool>,
    pub last_window: Property<Option<Address>>,
    pub last_window_title: Property<String>,
    pub persistent: Property<bool>,
}
