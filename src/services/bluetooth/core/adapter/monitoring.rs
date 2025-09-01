use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::Adapter;
use crate::services::{
    bluetooth::{
        BluetoothError,
        proxy::Adapter1Proxy,
        types::{AdapterRole, AddressType, PowerState, UUID},
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for Adapter {
    type Error = BluetoothError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = Adapter1Proxy::new(&self.zbus_connection, self.object_path.clone())
            .await
            .map_err(BluetoothError::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(BluetoothError::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        let cancel_token = cancellation_token.clone();

        tokio::spawn(async move {
            monitor(self, proxy, cancel_token).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
async fn monitor(
    adapter: Arc<Adapter>,
    proxy: Adapter1Proxy<'static>,
    cancellation_token: CancellationToken,
) {
    let mut address_changed = proxy.receive_address_changed().await;
    let mut address_type_changed = proxy.receive_address_type_changed().await;
    let mut name_changed = proxy.receive_name_changed().await;
    let mut alias_changed = proxy.receive_alias_changed().await;
    let mut class_changed = proxy.receive_class_changed().await;
    let mut connectable_changed = proxy.receive_connectable_changed().await;
    let mut powered_changed = proxy.receive_powered_changed().await;
    let mut power_state_changed = proxy.receive_power_state_changed().await;
    let mut discoverable_changed = proxy.receive_discoverable_changed().await;
    let mut discoverable_timeout_changed = proxy.receive_discoverable_timeout_changed().await;
    let mut discovering_changed = proxy.receive_discovering_changed().await;
    let mut pairable_changed = proxy.receive_pairable_changed().await;
    let mut pairable_timeout_changed = proxy.receive_pairable_timeout_changed().await;
    let mut uuids_changed = proxy.receive_uuids_changed().await;
    let mut modalias_changed = proxy.receive_modalias_changed().await;
    let mut roles_changed = proxy.receive_roles_changed().await;
    let mut experimental_features_changed = proxy.receive_experimental_features_changed().await;
    let mut manufacturer_changed = proxy.receive_manufacturer_changed().await;
    let mut version_changed = proxy.receive_version_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("AdapterMonitor cancelled for {}", adapter.object_path);
                return;
            }
            Some(change) = address_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.address.set(value);
                }
            }
            Some(change) = address_type_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.address_type.set(AddressType::from(value.as_str()));
                }
            }
            Some(change) = name_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.name.set(value);
                }
            }
            Some(change) = alias_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.alias.set(value);
                }
            }
            Some(change) = class_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.class.set(value);
                }
            }
            Some(change) = connectable_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.connectable.set(value);
                }
            }
            Some(change) = powered_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.powered.set(value);
                }
            }
            Some(change) = power_state_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.power_state.set(PowerState::from(value.as_str()));
                }
            }
            Some(change) = discoverable_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.discoverable.set(value);
                }
            }
            Some(change) = discoverable_timeout_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.discoverable_timeout.set(value);
                }
            }
            Some(change) = discovering_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.discovering.set(value);
                }
            }
            Some(change) = pairable_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.pairable.set(value);
                }
            }
            Some(change) = pairable_timeout_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.pairable_timeout.set(value);
                }
            }
            Some(change) = uuids_changed.next() => {
                if let Ok(value) = change.get().await {
                    let uuids: Vec<UUID> = value.into_iter()
                        .map(|s| UUID::from(s.as_str()))
                        .collect();
                    adapter.uuids.set(uuids);
                }
            }
            Some(change) = modalias_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.modalias.set(if value.is_empty() { None } else { Some(value) });
                }
            }
            Some(change) = roles_changed.next() => {
                if let Ok(value) = change.get().await {
                    let roles: Vec<AdapterRole> = value.into_iter()
                        .map(|s| AdapterRole::from(s.as_str()))
                        .collect();
                    adapter.roles.set(roles);
                }
            }
            Some(change) = experimental_features_changed.next() => {
                if let Ok(value) = change.get().await {
                    let features: Vec<UUID> = value.into_iter()
                        .map(|s| UUID::from(s.as_str()))
                        .collect();
                    adapter.experimental_features.set(features);
                }
            }
            Some(change) = manufacturer_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.manufacturer.set(value);
                }
            }
            Some(change) = version_changed.next() => {
                if let Ok(value) = change.get().await {
                    adapter.version.set(value);
                }
            }
            else => {
                debug!("All property streams ended for adapter {}", adapter.object_path);
                break;
            }
        }
    }

    debug!(
        "Property monitoring ended for adapter {}",
        adapter.object_path
    );
}
