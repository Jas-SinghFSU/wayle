mod types;

use std::sync::Arc;

use types::{LiveWorkspaceParams, WorkspaceParams};
use wayle_common::Property;
use wayle_traits::Reactive;

use crate::{Address, Error, MonitorId, WorkspaceData, WorkspaceId};

#[derive(Debug, Clone)]
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

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id.get() == other.id.get()
    }
}

impl Reactive for Workspace {
    type Error = Error;
    type Context<'a> = WorkspaceParams<'a>;
    type LiveContext<'a> = LiveWorkspaceParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let workspace_data = context.hypr_messenger.workspace(context.id).await?;

        Ok(Self::from_props(workspace_data))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let workspace_data = context.hypr_messenger.workspace(context.id).await?;
        let arc_workspace_data = Arc::new(Self::from_props(workspace_data));

        // TODO: Add monitoring
        Ok(arc_workspace_data)
    }
}

impl Workspace {
    pub(crate) fn from_props(workspace_data: WorkspaceData) -> Self {
        Self {
            id: Property::new(workspace_data.id),
            name: Property::new(workspace_data.name),
            monitor: Property::new(workspace_data.monitor),
            monitor_id: Property::new(workspace_data.monitor_id),
            windows: Property::new(workspace_data.windows),
            fullscreen: Property::new(workspace_data.fullscreen),
            last_window: Property::new(workspace_data.last_window),
            last_window_title: Property::new(workspace_data.last_window_title),
            persistent: Property::new(workspace_data.persistent),
        }
    }
}
