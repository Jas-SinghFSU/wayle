use std::{
    error::Error as StdError,
    sync::{Arc, Weak},
};

use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, warn};
use wayle_traits::ModelMonitoring;

use super::Monitor;
use crate::{
    Error,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

impl ModelMonitoring for Monitor {
    type Error = Error;
    #[instrument(skip(self), fields(name = %self.name.get()), err)]
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(cancel_token) = self.cancellation_token.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "monitor",
                resource_id: self.id.get().to_string(),
                missing_resource: "cancellation token",
            });
        };

        let Some(internal_tx) = self.internal_tx.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "monitor",
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
    weak_monitor: Weak<Monitor>,
    mut internal_rx: Receiver<ServiceNotification>,
    hypr_messenger: &HyprMessenger,
    cancellation_token: CancellationToken,
) {
    let hypr_messenger = hypr_messenger.clone();
    tokio::spawn(async move {
        loop {
            let Some(monitor) = weak_monitor.upgrade() else {
                return;
            };
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Monitor monitoring cancelled for monitor: {}", monitor.name.get());
                    return;
                }

                Ok(event) = internal_rx.recv() => {
                    match event {
                        ServiceNotification::MonitorUpdated(name) if name == monitor.name.get() => {
                            update_monitor(&hypr_messenger, &monitor).await;
                        }
                        ServiceNotification::WorkspaceFocused(workspace_id)
                        | ServiceNotification::WorkspaceUpdated(workspace_id) => {
                            let is_active = workspace_id == monitor.active_workspace.get().id;
                            let is_special = workspace_id == monitor.special_workspace.get().id;
                            if is_active || is_special {
                                update_monitor(&hypr_messenger, &monitor).await;
                            }
                        }
                        ServiceNotification::WorkspaceMoved(_) => {
                            update_monitor(&hypr_messenger, &monitor).await;
                        }
                        _ => {}
                    }
                }
                else => {
                    info!("Monitoring ended for monitor: {}", monitor.name.get());
                    return;
                }
            }
        }
    });
}

async fn update_monitor(hypr_messenger: &HyprMessenger, monitor: &Monitor) {
    let name = monitor.name.get();
    let monitor_data = match hypr_messenger.monitor(&name).await {
        Ok(monitor_data) => monitor_data,
        Err(e) => {
            warn!(
                error = %e,
                source = %StdError::source(&e).map(|s| s.to_string()).unwrap_or_default(),
                monitor_name = %name,
                "cannot get data for monitor"
            );
            return;
        }
    };

    monitor.update(monitor_data);
}
