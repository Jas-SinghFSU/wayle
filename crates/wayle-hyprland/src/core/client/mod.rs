mod monitoring;
pub(crate) mod types;

use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use types::{ClientParams, LiveClientParams};
use wayle_common::Property;
use wayle_traits::{ModelMonitoring, Reactive};

use crate::{
    Address, ClientData, ClientLocation, ClientSize, Error, FocusHistoryId, FullscreenMode,
    MonitorId, ProcessId, WorkspaceInfo,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) hypr_messenger: HyprMessenger,
    pub(crate) internal_tx: Option<Sender<ServiceNotification>>,
    pub(crate) cancellation_token: Option<CancellationToken>,

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

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.address.get() == other.address.get()
    }
}

impl Reactive for Client {
    type Error = Error;
    type Context<'a> = ClientParams<'a>;
    type LiveContext<'a> = LiveClientParams<'a>;

    #[instrument(skip(context), fields(address = %context.address), err)]
    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let client_data = context.hypr_messenger.client(&context.address).await?;
        Ok(Self::from_props(
            client_data,
            context.hypr_messenger,
            None,
            None,
        ))
    }

    #[instrument(skip(context), fields(address = %context.address), err)]
    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let client_data = context.hypr_messenger.client(&context.address).await?;
        let arc_client_data = Arc::new(Self::from_props(
            client_data,
            context.hypr_messenger,
            Some(context.internal_tx.clone()),
            Some(context.cancellation_token.child_token()),
        ));

        arc_client_data.clone().start_monitoring().await?;

        Ok(arc_client_data)
    }
}

impl Client {
    pub(crate) fn from_props(
        client_data: ClientData,
        hypr_messenger: &HyprMessenger,
        internal_tx: Option<Sender<ServiceNotification>>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            hypr_messenger: hypr_messenger.clone(),
            internal_tx,
            cancellation_token,
            address: Property::new(client_data.address),
            mapped: Property::new(client_data.mapped),
            hidden: Property::new(client_data.hidden),
            at: Property::new(client_data.at),
            size: Property::new(client_data.size),
            workspace: Property::new(client_data.workspace),
            floating: Property::new(client_data.floating),
            pseudo: Property::new(client_data.pseudo),
            monitor: Property::new(client_data.monitor),
            class: Property::new(client_data.class),
            title: Property::new(client_data.title),
            initial_class: Property::new(client_data.initial_class),
            initial_title: Property::new(client_data.initial_title),
            pid: Property::new(client_data.pid),
            xwayland: Property::new(client_data.xwayland),
            pinned: Property::new(client_data.pinned),
            fullscreen: Property::new(client_data.fullscreen),
            fullscreen_client: Property::new(client_data.fullscreen_client),
            grouped: Property::new(client_data.grouped),
            tags: Property::new(client_data.tags),
            swallowing: Property::new(client_data.swallowing),
            focus_history_id: Property::new(client_data.focus_history_id),
            inhibiting_idle: Property::new(client_data.inhibiting_idle),
            xdg_tag: Property::new(client_data.xdg_tag),
            xdg_description: Property::new(client_data.xdg_description),
        }
    }

    pub(crate) fn update(&self, client_data: ClientData) {
        self.address.set(client_data.address);
        self.mapped.set(client_data.mapped);
        self.hidden.set(client_data.hidden);
        self.at.set(client_data.at);
        self.size.set(client_data.size);
        self.workspace.set(client_data.workspace);
        self.floating.set(client_data.floating);
        self.pseudo.set(client_data.pseudo);
        self.monitor.set(client_data.monitor);
        self.class.set(client_data.class);
        self.title.set(client_data.title);
        self.initial_class.set(client_data.initial_class);
        self.initial_title.set(client_data.initial_title);
        self.pid.set(client_data.pid);
        self.xwayland.set(client_data.xwayland);
        self.pinned.set(client_data.pinned);
        self.fullscreen.set(client_data.fullscreen);
        self.fullscreen_client.set(client_data.fullscreen_client);
        self.grouped.set(client_data.grouped);
        self.tags.set(client_data.tags);
        self.swallowing.set(client_data.swallowing);
        self.focus_history_id.set(client_data.focus_history_id);
        self.inhibiting_idle.set(client_data.inhibiting_idle);
        self.xdg_tag.set(client_data.xdg_tag);
        self.xdg_description.set(client_data.xdg_description);
    }
}
