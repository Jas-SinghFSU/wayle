use std::sync::Arc;

use tracing::{error, instrument};

use crate::{
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    ipc::HyprMessenger,
};

pub(super) struct HyprlandDiscovery {
    pub workspaces: Vec<Arc<Workspace>>,
    pub clients: Vec<Arc<Client>>,
    pub monitors: Vec<Arc<Monitor>>,
    pub layers: Vec<Layer>,
}

impl HyprlandDiscovery {
    #[instrument(skip(hypr_messenger))]
    pub async fn new(hypr_messenger: HyprMessenger) -> Self {
        let all_layers = hypr_messenger.layers().await.unwrap_or_else(|e| {
            error!(error = %e, "cannot discover layers");
            vec![]
        });

        let all_clients = hypr_messenger.clients().await.unwrap_or_else(|e| {
            error!(error = %e, "cannot discover clients");
            vec![]
        });

        let all_monitors = hypr_messenger.monitors().await.unwrap_or_else(|e| {
            error!(error = %e, "cannot discover monitors");
            vec![]
        });

        let all_workspaces = hypr_messenger.workspaces().await.unwrap_or_else(|e| {
            error!(error = %e, "cannot discover workspaces");
            vec![]
        });

        let mut clients = Vec::new();
        let mut monitors = Vec::new();
        let mut workspaces = Vec::new();
        let layers = all_layers.into_iter().map(Layer::from_props).collect();

        for client_data in all_clients {
            clients.push(Arc::new(Client::from_props(client_data)));
        }

        for monitor_data in all_monitors {
            monitors.push(Arc::new(Monitor::from_props(monitor_data)));
        }

        for workspace_data in all_workspaces {
            workspaces.push(Arc::new(Workspace::from_props(workspace_data)));
        }

        Self {
            workspaces,
            clients,
            monitors,
            layers,
        }
    }
}
