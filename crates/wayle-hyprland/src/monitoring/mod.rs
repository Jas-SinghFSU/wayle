mod layer;
mod monitor;
mod window;
mod workspace;

use layer::{handle_layer_created, handle_layer_removed};
use monitor::{handle_monitor_created, handle_monitor_removed, handle_monitor_updated};
use tokio::sync::broadcast::Receiver;
use wayle_traits::ServiceMonitoring;
use window::{
    handle_active_window_updated, handle_window_created, handle_window_moved,
    handle_window_removed, handle_window_updated,
};
use workspace::{
    handle_workspace_created, handle_workspace_focused, handle_workspace_moved,
    handle_workspace_removed, handle_workspace_updated,
};

use crate::{Error, HyprlandService, ServiceNotification};

impl ServiceMonitoring for HyprlandService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let internal_rx = self.internal_tx.subscribe();

        handle_internal_events(internal_rx).await;

        Ok(())
    }
}

async fn handle_internal_events(mut internal_rx: Receiver<ServiceNotification>) {
    tokio::spawn(async move {
        while let Ok(event) = internal_rx.recv().await {
            match event {
                ServiceNotification::WorkspaceCreated(id) => {
                    handle_workspace_created();
                }
                ServiceNotification::WorkspaceUpdated(id) => {
                    handle_workspace_updated();
                }
                ServiceNotification::WorkspaceRemoved(id) => {
                    handle_workspace_removed();
                }
                ServiceNotification::WorkspaceFocused(id) => {
                    handle_workspace_focused();
                }
                ServiceNotification::WorkspaceMoved(id) => {
                    handle_workspace_moved();
                }

                ServiceNotification::MonitorCreated(name) => {
                    handle_monitor_created();
                }
                ServiceNotification::MonitorUpdated(name) => {
                    handle_monitor_updated();
                }
                ServiceNotification::MonitorRemoved(name) => {
                    handle_monitor_removed();
                }

                ServiceNotification::WindowCreated(address) => {
                    handle_window_created();
                }
                ServiceNotification::WindowUpdated(address) => {
                    handle_window_updated();
                }
                ServiceNotification::WindowRemoved(address) => {
                    handle_window_removed();
                }
                ServiceNotification::ActiveWindowUpdated(address) => {
                    handle_active_window_updated();
                }
                ServiceNotification::WindowMoved(address, workspace_id) => {
                    handle_window_moved();
                }

                ServiceNotification::LayerCreated(namespace) => {
                    handle_layer_created();
                }
                ServiceNotification::LayerRemoved(namespace) => {
                    handle_layer_removed();
                }
            }
        }
    });
}
