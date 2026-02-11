mod plan;
mod projector;
mod reconcile;

use std::sync::Arc;

use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;
use wayle_traits::ServiceMonitoring;

use crate::{
    Error, HyprlandEvent, HyprlandService,
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    ipc::HyprMessenger,
};

#[derive(Clone)]
pub(super) struct SyncRuntime {
    pub(super) event_tx: Sender<HyprlandEvent>,
    pub(super) hyprland_tx: Sender<HyprlandEvent>,
    pub(super) hypr_messenger: HyprMessenger,
    pub(super) clients: Property<Vec<Arc<Client>>>,
    pub(super) monitors: Property<Vec<Arc<Monitor>>>,
    pub(super) workspaces: Property<Vec<Arc<Workspace>>>,
    pub(super) layers: Property<Vec<Layer>>,
    pub(super) cancellation_token: CancellationToken,
}

impl ServiceMonitoring for HyprlandService {
    type Error = Error;

    #[instrument(skip(self), err)]
    async fn start_monitoring(&self) -> std::result::Result<(), Self::Error> {
        projector::spawn(SyncRuntime {
            event_tx: self.event_tx.clone(),
            hyprland_tx: self.hyprland_tx.clone(),
            hypr_messenger: self.hypr_messenger.clone(),
            clients: self.clients.clone(),
            monitors: self.monitors.clone(),
            workspaces: self.workspaces.clone(),
            layers: self.layers.clone(),
            cancellation_token: self.cancellation_token.clone(),
        });

        Ok(())
    }
}
