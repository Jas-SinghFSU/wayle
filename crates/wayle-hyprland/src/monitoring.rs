use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{error, instrument, warn};
use wayle_common::Property;
use wayle_traits::{Reactive, ServiceMonitoring};

use crate::{
    Address, Error, HyprlandEvent, HyprlandService, WorkspaceId,
    core::{
        client::{Client, types::LiveClientParams},
        layer::Layer,
        monitor::{Monitor, types::LiveMonitorParams},
        workspace::{Workspace, types::LiveWorkspaceParams},
    },
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

impl ServiceMonitoring for HyprlandService {
    type Error = Error;

    #[instrument(skip(self), err)]
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        handle_internal_events(
            &self.internal_tx,
            &self.hyprland_tx,
            &self.hypr_messenger,
            &self.clients,
            &self.monitors,
            &self.workspaces,
            &self.layers,
            &self.cancellation_token,
        )
        .await;

        Ok(())
    }
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
async fn handle_internal_events(
    internal_tx: &Sender<ServiceNotification>,
    hyprland_tx: &Sender<HyprlandEvent>,
    hypr_messenger: &HyprMessenger,
    clients: &Property<Vec<Arc<Client>>>,
    monitors: &Property<Vec<Arc<Monitor>>>,
    workspaces: &Property<Vec<Arc<Workspace>>>,
    layers: &Property<Vec<Layer>>,
    cancellation_token: &CancellationToken,
) {
    let internal_tx = internal_tx.clone();
    let hyprland_tx = hyprland_tx.clone();
    let hypr_messenger = hypr_messenger.clone();
    let clients = clients.clone();
    let monitors = monitors.clone();
    let workspaces = workspaces.clone();
    let layers = layers.clone();
    let cancellation_token = cancellation_token.clone();

    let mut internal_rx = internal_tx.subscribe();
    let mut hyprland_rx = hyprland_tx.subscribe();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    return;
                }
                Ok(event) = internal_rx.recv() => {
                    match event {
                        ServiceNotification::WorkspaceCreated(id) => {
                            handle_workspace_created(
                                id,
                                &workspaces,
                                &hypr_messenger,
                                &internal_tx,
                                &cancellation_token
                            ).await;
                        }
                        ServiceNotification::WorkspaceRemoved(id) => {
                            handle_workspace_removed(
                                id,
                                &workspaces,
                            ).await;
                        }

                        ServiceNotification::MonitorCreated(name) => {
                            handle_monitor_created(
                                name,
                                &monitors,
                                &hypr_messenger,
                                &internal_tx,
                                &cancellation_token
                            ).await;
                        }
                        ServiceNotification::MonitorRemoved(name) => {
                            handle_monitor_removed(
                                name,
                                &monitors,
                            ).await;
                        }

                        ServiceNotification::ClientCreated(address) => {
                            handle_client_created(
                                address,
                                &clients,
                                &hypr_messenger,
                                &internal_tx,
                                &cancellation_token
                            ).await;
                        }
                        ServiceNotification::ClientRemoved(address) => {
                            handle_client_removed(
                                address,
                                &clients,
                            ).await;
                        }

                        ServiceNotification::LayerCreated(namespace) => {
                            handle_layer_created(
                                namespace,
                                &layers,
                                &hypr_messenger,
                            ).await;
                        }
                        ServiceNotification::LayerRemoved(namespace) => {
                            handle_layer_removed(
                                namespace,
                                &layers,
                            ).await;
                        }

                        _ => { /* remaining events handled by core models */ }
                    }
                }
                Ok(event) = hyprland_rx.recv() => {
                    let HyprlandEvent::Fullscreen { .. } = event else { continue };
                    let Ok(window) = hypr_messenger.active_window().await else { continue };
                    let _ = internal_tx.send(ServiceNotification::ClientUpdated(window.address));
                }
                else => {
                    break;
                }
            }
        }
    });
}

pub(super) async fn handle_workspace_created(
    id: WorkspaceId,
    workspaces: &Property<Vec<Arc<Workspace>>>,
    hypr_messenger: &HyprMessenger,
    internal_tx: &Sender<ServiceNotification>,
    cancellation_token: &CancellationToken,
) {
    let workspace = match Workspace::get_live(LiveWorkspaceParams {
        id,
        hypr_messenger,
        internal_tx,
        cancellation_token,
    })
    .await
    {
        Ok(workspace) => workspace,
        Err(e) => {
            error!("Failed to get workspace with id '{id}': {e}");
            return;
        }
    };

    let mut updated_workspaces = workspaces.get();
    updated_workspaces.push(workspace);
    workspaces.set(updated_workspaces);
}

pub(super) async fn handle_workspace_removed(
    id: WorkspaceId,
    workspaces: &Property<Vec<Arc<Workspace>>>,
) {
    let mut updated_workspaces = workspaces.get();
    let Some(workspace) = updated_workspaces
        .iter()
        .find(|workspace| workspace.id.get() == id)
    else {
        warn!("Failed to remove workspace with id '{id}': Not Found");
        return;
    };

    if let Some(cancel_token) = workspace.cancellation_token.as_ref() {
        cancel_token.cancel();
    }

    updated_workspaces.retain(|workspace| workspace.id.get() != id);
    workspaces.set(updated_workspaces);
}

pub(super) async fn handle_monitor_created(
    name: String,
    monitors: &Property<Vec<Arc<Monitor>>>,
    hypr_messenger: &HyprMessenger,
    internal_tx: &Sender<ServiceNotification>,
    cancellation_token: &CancellationToken,
) {
    let monitor = match Monitor::get_live(LiveMonitorParams {
        name: name.clone(),
        hypr_messenger,
        internal_tx,
        cancellation_token,
    })
    .await
    {
        Ok(monitor) => monitor,
        Err(e) => {
            error!("Failed to get monitor with name '{name}': {e}");
            return;
        }
    };

    let mut updated_monitors = monitors.get();
    updated_monitors.push(monitor);
    monitors.set(updated_monitors);
}

pub(super) async fn handle_monitor_removed(name: String, monitors: &Property<Vec<Arc<Monitor>>>) {
    let mut updated_monitors = monitors.get();
    let Some(monitor) = updated_monitors
        .iter()
        .find(|monitor| monitor.name.get() == name)
    else {
        warn!("Failed to remove monitor with name '{name}': Not Found");
        return;
    };

    if let Some(cancel_token) = monitor.cancellation_token.as_ref() {
        cancel_token.cancel();
    }

    updated_monitors.retain(|monitor| monitor.name.get() != name);
    monitors.set(updated_monitors);
}

pub(super) async fn handle_client_created(
    address: Address,
    clients: &Property<Vec<Arc<Client>>>,
    hypr_messenger: &HyprMessenger,
    internal_tx: &Sender<ServiceNotification>,
    cancellation_token: &CancellationToken,
) {
    let client = match Client::get_live(LiveClientParams {
        address: address.clone(),
        hypr_messenger,
        internal_tx,
        cancellation_token,
    })
    .await
    {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to get client with address '{address}': {e}");
            return;
        }
    };

    let mut updated_clients = clients.get();
    updated_clients.push(client);
    clients.set(updated_clients);
}

pub(super) async fn handle_client_removed(address: Address, clients: &Property<Vec<Arc<Client>>>) {
    let mut updated_clients = clients.get();
    let Some(client) = updated_clients
        .iter()
        .find(|client| client.address.get() == address)
    else {
        warn!("Failed to remove client with address '{address}': Not Found");
        return;
    };

    if let Some(cancel_token) = client.cancellation_token.as_ref() {
        cancel_token.cancel();
    }

    updated_clients.retain(|client| client.address.get() != address);
    clients.set(updated_clients);
}

pub(super) async fn handle_layer_created(
    namespace: String,
    layers: &Property<Vec<Layer>>,
    hypr_messenger: &HyprMessenger,
) {
    let all_layers = match hypr_messenger.layers().await {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to query layers: {e}");
            return;
        }
    };

    let current_layers = layers.get();
    let current_addresses: Vec<_> = current_layers
        .iter()
        .map(|layer| layer.address.get())
        .collect();

    let new_layers: Vec<Layer> = all_layers
        .into_iter()
        .filter(|layer_data| layer_data.namespace == namespace)
        .filter(|layer_data| !current_addresses.contains(&layer_data.address))
        .map(Layer::from_props)
        .collect();

    if !new_layers.is_empty() {
        let mut updated_layers = current_layers;
        updated_layers.extend(new_layers);
        layers.set(updated_layers);
    }
}

pub(super) async fn handle_layer_removed(namespace: String, layers: &Property<Vec<Layer>>) {
    let mut updated_layers = layers.get();
    let original_len = updated_layers.len();
    updated_layers.retain(|layer| layer.namespace.get() != namespace);

    if updated_layers.len() != original_len {
        layers.set(updated_layers);
    } else {
        warn!("Failed to remove layer with namespace '{namespace}': Not Found");
    }
}
