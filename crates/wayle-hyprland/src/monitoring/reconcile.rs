//! State reconciliation via Hyprland IPC queries.
//!
//! Queries live state from Hyprland and reconciles with the in-memory model,
//! preserving existing instances where possible.

use std::{collections::HashMap, sync::Arc};

use tracing::{instrument, warn};

use super::{SyncRuntime, plan};
use crate::core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace};

#[instrument(skip_all, fields(
    sync_monitors = plan.monitors,
    sync_workspaces = plan.workspaces,
    sync_clients = plan.clients,
    sync_layers = plan.layers,
))]
pub(super) async fn sync_model_state(runtime: &SyncRuntime, plan: plan::SyncPlan) {
    if plan.monitors {
        sync_monitors_state(runtime).await;
    }

    if plan.workspaces {
        sync_workspaces_state(runtime).await;
    }

    if plan.clients {
        sync_clients_state(runtime).await;
    }

    if plan.layers {
        sync_layers_state(runtime).await;
    }
}

async fn sync_clients_state(runtime: &SyncRuntime) {
    let live_clients = match runtime.hypr_messenger.clients().await {
        Ok(data) => data,
        Err(e) => {
            warn!(error = %e, "cannot query clients while syncing model state");
            return;
        }
    };

    let current_clients = runtime.clients.get();
    let mut current_by_address: HashMap<_, _> = current_clients
        .iter()
        .map(|client| (client.address.get(), Arc::clone(client)))
        .collect();

    let mut reconciled = Vec::with_capacity(live_clients.len());
    for client_data in live_clients {
        let address = client_data.address.clone();
        if let Some(client) = current_by_address.remove(&address) {
            client.update(client_data);
            reconciled.push(client);
            continue;
        }

        reconciled.push(Arc::new(Client::from_props(client_data)));
    }
    if reconciled != current_clients {
        runtime.clients.set(reconciled);
    }
}

async fn sync_workspaces_state(runtime: &SyncRuntime) {
    let live_workspaces = match runtime.hypr_messenger.workspaces().await {
        Ok(data) => data,
        Err(e) => {
            warn!(error = %e, "cannot query workspaces while syncing model state");
            return;
        }
    };

    let current_workspaces = runtime.workspaces.get();
    let mut current_by_id: HashMap<_, _> = current_workspaces
        .iter()
        .map(|workspace| (workspace.id.get(), Arc::clone(workspace)))
        .collect();

    let mut reconciled = Vec::with_capacity(live_workspaces.len());
    for workspace_data in live_workspaces {
        let id = workspace_data.id;
        if let Some(workspace) = current_by_id.remove(&id) {
            workspace.update(workspace_data);
            reconciled.push(workspace);
            continue;
        }

        reconciled.push(Arc::new(Workspace::from_props(workspace_data)));
    }
    if reconciled != current_workspaces {
        runtime.workspaces.set(reconciled);
    }
}

async fn sync_monitors_state(runtime: &SyncRuntime) {
    let live_monitors = match runtime.hypr_messenger.monitors().await {
        Ok(data) => data,
        Err(e) => {
            warn!(error = %e, "cannot query monitors while syncing model state");
            return;
        }
    };

    let current_monitors = runtime.monitors.get();
    let mut current_by_name: HashMap<_, _> = current_monitors
        .iter()
        .map(|monitor| (monitor.name.get(), Arc::clone(monitor)))
        .collect();

    let mut reconciled = Vec::with_capacity(live_monitors.len());
    for monitor_data in live_monitors {
        let name = monitor_data.name.clone();
        if let Some(monitor) = current_by_name.remove(&name) {
            monitor.update(monitor_data);
            reconciled.push(monitor);
            continue;
        }

        reconciled.push(Arc::new(Monitor::from_props(monitor_data)));
    }
    if reconciled != current_monitors {
        runtime.monitors.set(reconciled);
    }
}

async fn sync_layers_state(runtime: &SyncRuntime) {
    let live_layers = match runtime.hypr_messenger.layers().await {
        Ok(data) => data.into_iter().map(Layer::from_props).collect::<Vec<_>>(),
        Err(e) => {
            warn!(error = %e, "cannot query layers while syncing model state");
            return;
        }
    };

    if live_layers != runtime.layers.get() {
        runtime.layers.set(live_layers);
    }
}
