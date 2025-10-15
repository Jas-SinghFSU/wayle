use std::sync::Arc;

use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{
    Address, Error, HyprlandService, WorkspaceId,
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    ipc::events::types::ServiceNotification,
};

impl ServiceMonitoring for HyprlandService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let internal_rx = self.internal_tx.subscribe();

        handle_internal_events(
            internal_rx,
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

async fn handle_internal_events(
    mut internal_rx: Receiver<ServiceNotification>,
    clients: &Property<Vec<Arc<Client>>>,
    monitors: &Property<Vec<Arc<Monitor>>>,
    workspaces: &Property<Vec<Arc<Workspace>>>,
    layers: &Property<Vec<Arc<Layer>>>,
    cancellation_token: &CancellationToken,
) {
    let clients = clients.clone();
    let monitors = monitors.clone();
    let workspaces = workspaces.clone();
    let layers = layers.clone();
    let cancellation_token = cancellation_token.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Hyprland service monitoring cancelled");
                    return;
                }
                Ok(event) = internal_rx.recv() => {
                    match event {
                        ServiceNotification::WorkspaceCreated(id) => {
                            handle_workspace_created(id, &clients);
                        }
                        ServiceNotification::WorkspaceRemoved(id) => {
                            handle_workspace_removed(id, &clients);
                        }

                        ServiceNotification::MonitorCreated(name) => {
                            handle_monitor_created(name, &monitors);
                        }
                        ServiceNotification::MonitorRemoved(name) => {
                            handle_monitor_removed(name, &monitors);
                        }

                        ServiceNotification::ClientCreated(address) => {
                            handle_client_created(address, &clients);
                        }
                        ServiceNotification::ClientRemoved(address) => {
                            handle_client_removed(address, &clients);
                        }

                        ServiceNotification::LayerCreated(namespace) => {
                            handle_layer_created(namespace, &layers);
                        }
                        ServiceNotification::LayerRemoved(namespace) => {
                            handle_layer_removed(namespace, &layers);
                        }

                        _ => { /* remaining events handled by core models */ }
                    }
                }
                else => {
                    debug!("All property streams ended for hyprland service");
                    break;
                }
            }
        }
    });
}

pub(super) fn handle_workspace_created(id: WorkspaceId, clients: &Property<Vec<Arc<Client>>>) {
    todo!()
}

pub(super) fn handle_workspace_removed(id: WorkspaceId, clients: &Property<Vec<Arc<Client>>>) {
    todo!()
}

pub(super) fn handle_monitor_created(name: String, monitors: &Property<Vec<Arc<Monitor>>>) {
    todo!()
}

pub(super) fn handle_monitor_removed(name: String, monitors: &Property<Vec<Arc<Monitor>>>) {
    todo!()
}

pub(super) fn handle_client_created(address: Address, clients: &Property<Vec<Arc<Client>>>) {
    todo!()
}

pub(super) fn handle_client_removed(address: Address, clients: &Property<Vec<Arc<Client>>>) {
    todo!()
}

pub(super) fn handle_layer_created(namespace: String, layers: &Property<Vec<Arc<Layer>>>) {
    todo!()
}

pub(super) fn handle_layer_removed(namespace: String, layers: &Property<Vec<Arc<Layer>>>) {
    todo!()
}
