mod monitoring;
pub(crate) mod types;

use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use types::{LiveWorkspaceParams, WorkspaceParams};
use wayle_common::Property;
use wayle_traits::{ModelMonitoring, Reactive};

use crate::{
    Address, Error, MonitorId, WorkspaceData, WorkspaceId,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub(crate) hypr_messenger: HyprMessenger,
    pub(crate) internal_tx: Option<Sender<ServiceNotification>>,
    pub(crate) cancellation_token: Option<CancellationToken>,

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

    #[instrument(skip(context), fields(id = %context.id), err)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let workspace_data = context.hypr_messenger.workspace(context.id).await?;

        Ok(Self::from_props(
            workspace_data,
            context.hypr_messenger,
            None,
            None,
        ))
    }

    #[instrument(skip(context), fields(id = %context.id), err)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let workspace_data = context.hypr_messenger.workspace(context.id).await?;
        let arc_workspace = Arc::new(Self::from_props(
            workspace_data,
            context.hypr_messenger,
            Some(context.internal_tx.clone()),
            Some(context.cancellation_token.child_token()),
        ));

        arc_workspace.clone().start_monitoring().await?;

        Ok(arc_workspace)
    }
}

impl Workspace {
    pub(crate) fn from_props(
        workspace_data: WorkspaceData,
        hypr_messenger: &HyprMessenger,
        internal_tx: Option<Sender<ServiceNotification>>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            hypr_messenger: hypr_messenger.clone(),
            internal_tx,
            cancellation_token,
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

    pub(crate) fn update(&self, workspace_data: WorkspaceData) {
        self.id.set(workspace_data.id);
        self.name.set(workspace_data.name);
        self.monitor.set(workspace_data.monitor);
        self.monitor_id.set(workspace_data.monitor_id);
        self.windows.set(workspace_data.windows);
        self.fullscreen.set(workspace_data.fullscreen);
        self.last_window.set(workspace_data.last_window);
        self.last_window_title.set(workspace_data.last_window_title);
        self.persistent.set(workspace_data.persistent);
    }
}
