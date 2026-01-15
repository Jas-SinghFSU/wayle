use std::sync::{Arc, Weak};

use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, warn};
use wayle_traits::ModelMonitoring;

use super::Workspace;
use crate::{
    Error, WorkspaceId,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

impl ModelMonitoring for Workspace {
    type Error = Error;
    #[instrument(skip(self), fields(id = %self.id.get()), err)]
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(cancel_token) = self.cancellation_token.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "workspace",
                resource_id: self.id.get().to_string(),
                missing_resource: "cancellation token",
            });
        };

        let Some(internal_tx) = self.internal_tx.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "workspace",
                resource_id: self.id.get().to_string(),
                missing_resource: "internal transmitter",
            });
        };

        let weak_self = Arc::downgrade(&self);
        let internal_rx = internal_tx.subscribe();

        start_monitoring(
            weak_self,
            internal_rx,
            &self.hypr_messenger,
            cancel_token.child_token(),
        );

        Ok(())
    }
}

fn start_monitoring(
    weak_workspace: Weak<Workspace>,
    mut internal_rx: Receiver<ServiceNotification>,
    hypr_messenger: &HyprMessenger,
    cancellation_token: CancellationToken,
) {
    let hypr_messenger = hypr_messenger.clone();
    tokio::spawn(async move {
        loop {
            let Some(workspace) = weak_workspace.upgrade() else {
                return;
            };
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Workspace monitoring cancelled for workspace: {}", workspace.id.get());
                    return;
                }

                Ok(event) = internal_rx.recv() => {
                    match event {
                        ServiceNotification::WorkspaceUpdated(id)
                        | ServiceNotification::WorkspaceMoved(id) => {
                            if id == workspace.id.get() {
                                update_workspace(id, &hypr_messenger, &workspace).await;
                            }
                        },
                        ServiceNotification::ClientCreated(_)
                        | ServiceNotification::ClientRemoved(_) => {
                            let id = workspace.id.get();
                            update_workspace(id, &hypr_messenger, &workspace).await;
                        },
                        ServiceNotification::ClientMoved(_, workspace_id) => {
                            if workspace_id == workspace.id.get() {
                                let id = workspace.id.get();
                                update_workspace(id, &hypr_messenger, &workspace).await;
                            }
                        },
                        _ => {}
                    }
                }
                else => {
                    info!("Monitoring ended for workspace: {}", workspace.id.get());
                    return;
                }
            }
        }
    });
}

async fn update_workspace(id: WorkspaceId, hypr_messenger: &HyprMessenger, workspace: &Workspace) {
    let workspace_data = match hypr_messenger.workspace(id).await {
        Ok(ws_data) => ws_data,
        Err(e) => {
            warn!(error = %e, workspace_id = id, "cannot get data for workspace");
            return;
        }
    };

    workspace.update(workspace_data);
}
