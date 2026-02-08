use std::{
    error::Error as StdError,
    sync::{Arc, Weak},
};

use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, warn};
use wayle_traits::ModelMonitoring;

use super::Client;
use crate::{
    Address, Error,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

impl ModelMonitoring for Client {
    type Error = Error;
    #[instrument(skip(self), fields(address = %self.address.get()), err)]
    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(cancel_token) = self.cancellation_token.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "client",
                resource_id: self.address.get().to_string(),
                missing_resource: "cancellation token",
            });
        };

        let Some(internal_tx) = self.internal_tx.as_ref() else {
            return Err(Error::MonitoringSetupError {
                resource_type: "client",
                resource_id: self.address.get().to_string(),
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
    weak_client: Weak<Client>,
    mut internal_rx: Receiver<ServiceNotification>,
    hypr_messenger: &HyprMessenger,
    cancellation_token: CancellationToken,
) {
    let hypr_messenger = hypr_messenger.clone();
    tokio::spawn(async move {
        loop {
            let Some(client) = weak_client.upgrade() else {
                return;
            };
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Client monitoring cancelled for client: {}", client.address.get());
                    return;
                }

                Ok(event) = internal_rx.recv() => {
                    match event {
                        ServiceNotification::ClientUpdated(address) | ServiceNotification::ClientMoved(address, _) => {
                            if address == client.address.get() {
                                update_client(address, &hypr_messenger, &client).await;
                            }
                        },
                        ServiceNotification::WorkspaceMoved(workspace_id) => {
                            if client.workspace.get().id == workspace_id {
                                update_client(client.address.get(), &hypr_messenger, &client).await;
                            }
                        },
                        _ => {}
                    }
                }
                else => {
                    info!("Monitoring ended for client: {}", client.address.get());
                    return;
                }
            }
        }
    });
}

async fn update_client(address: Address, hypr_messenger: &HyprMessenger, client: &Client) {
    let client_data = match hypr_messenger.client(&address).await {
        Ok(client_data) => client_data,
        Err(e) => {
            warn!(
                error = %e,
                source = %StdError::source(&e).map(|s| s.to_string()).unwrap_or_default(),
                client_address = %address,
                "cannot get data for client"
            );
            return;
        }
    };

    client.update(client_data);
}
