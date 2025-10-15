mod types;

use std::sync::Arc;

use types::{ClientParams, LiveClientParams};
use wayle_common::Property;
use wayle_traits::Reactive;

use crate::{
    Address, ClientData, ClientLocation, ClientSize, Error, FocusHistoryId, FullscreenMode,
    MonitorId, ProcessId, WorkspaceInfo,
};

#[derive(Debug, Clone)]
pub struct Client {
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

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let client_data = context.hypr_messenger.client(context.address).await?;
        Ok(Self::from_props(client_data))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let client_data = context.hypr_messenger.client(context.address).await?;
        let arc_client_data = Arc::new(Self::from_props(client_data));

        //TODO: Add monitoring here
        Ok(arc_client_data)
    }
}

impl Client {
    pub(crate) fn from_props(client_data: ClientData) -> Self {
        Self {
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
}
