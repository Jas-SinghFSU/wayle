use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{error, instrument};
use wayle_traits::ModelMonitoring;

use crate::{
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

pub(super) struct HyprlandDiscovery {
    pub workspaces: Vec<Arc<Workspace>>,
    pub clients: Vec<Arc<Client>>,
    pub monitors: Vec<Arc<Monitor>>,
    pub layers: Vec<Layer>,
}

impl HyprlandDiscovery {
    #[instrument(skip(hypr_messenger, internal_tx, cancellation_token))]
    pub async fn new(
        hypr_messenger: HyprMessenger,
        internal_tx: &Sender<ServiceNotification>,
        cancellation_token: &CancellationToken,
    ) -> Self {
        let all_layers = hypr_messenger.layers().await.unwrap_or_else(|e| {
            error!("Failed to discover layers: {e}");
            vec![]
        });

        let all_clients = hypr_messenger.clients().await.unwrap_or_else(|e| {
            error!("Failed to discover clients: {e}");
            vec![]
        });

        let all_monitors = hypr_messenger.monitors().await.unwrap_or_else(|e| {
            error!("Failed to discover monitors: {e}");
            vec![]
        });

        let all_workspaces = hypr_messenger.workspaces().await.unwrap_or_else(|e| {
            error!("Failed to discover workspaces: {e}");
            vec![]
        });

        let mut clients = Vec::new();
        let mut monitors = Vec::new();
        let mut workspaces = Vec::new();
        let layers = all_layers.into_iter().map(Layer::from_props).collect();

        for client_data in all_clients {
            let client_address = client_data.address.clone();
            let client = Arc::new(Client::from_props(
                client_data,
                &hypr_messenger,
                Some(internal_tx.clone()),
                Some(cancellation_token.child_token()),
            ));

            match client.clone().start_monitoring().await {
                Ok(_) => clients.push(client),
                Err(e) => {
                    error!(
                        "Failed to start monitoring for client '{client_address}': {e}... Discarding."
                    )
                }
            }
        }

        for monitor_data in all_monitors {
            let monitor_name = monitor_data.name.clone();
            let monitor = Arc::new(Monitor::from_props(
                monitor_data,
                &hypr_messenger,
                Some(internal_tx.clone()),
                Some(cancellation_token.child_token()),
            ));

            match monitor.clone().start_monitoring().await {
                Ok(_) => monitors.push(monitor),
                Err(e) => {
                    error!(
                        "Failed to start monitoring for monitor '{monitor_name}': {e}... Discarding."
                    )
                }
            }
        }

        for workspace_data in all_workspaces {
            let workspace_id = workspace_data.id;
            let workspace = Arc::new(Workspace::from_props(
                workspace_data,
                &hypr_messenger,
                Some(internal_tx.clone()),
                Some(cancellation_token.child_token()),
            ));

            match workspace.clone().start_monitoring().await {
                Ok(_) => workspaces.push(workspace),
                Err(e) => {
                    error!(
                        "Failed to start monitoring for workspace '{workspace_id}': {e}... Discarding."
                    )
                }
            }
        }

        Self {
            workspaces,
            clients,
            monitors,
            layers,
        }
    }
}
