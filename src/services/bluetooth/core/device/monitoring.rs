use std::sync::Arc;

use futures::StreamExt;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::Device;
use crate::services::{
    bluetooth::{
        BluetoothError,
        proxy::Device1Proxy,
        types::{AddressType, PreferredBearer, ServiceNotification},
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for Device {
    type Error = BluetoothError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = Device1Proxy::new(&self.zbus_connection, self.object_path.clone())
            .await
            .map_err(BluetoothError::DbusError)?;

        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(BluetoothError::OperationFailed {
                operation: "start_monitoring",
                reason: "A cancellation_token was not found.".to_string(),
            });
        };

        let cancel_token = cancellation_token.clone();
        let notifier_tx = self.notifier_tx.clone();

        tokio::spawn(async move {
            monitor(self, proxy, cancel_token, notifier_tx).await;
        });

        Ok(())
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
async fn monitor(
    device: Arc<Device>,
    proxy: Device1Proxy<'static>,
    cancellation_token: CancellationToken,
    notifier_tx: broadcast::Sender<ServiceNotification>,
) {
    let mut address_changed = proxy.receive_address_changed().await;
    let mut address_type_changed = proxy.receive_address_type_changed().await;
    let mut name_changed = proxy.receive_name_changed().await;
    let mut icon_changed = proxy.receive_icon_changed().await;
    let mut class_changed = proxy.receive_class_changed().await;
    let mut appearance_changed = proxy.receive_appearance_changed().await;
    let mut uuids_changed = proxy.receive_uuids_changed().await;
    let mut paired_changed = proxy.receive_paired_changed().await;
    let mut bonded_changed = proxy.receive_bonded_changed().await;
    let mut connected_changed = proxy.receive_connected_changed().await;
    let mut trusted_changed = proxy.receive_trusted_changed().await;
    let mut blocked_changed = proxy.receive_blocked_changed().await;
    let mut wake_allowed_changed = proxy.receive_wake_allowed_changed().await;
    let mut alias_changed = proxy.receive_alias_changed().await;
    let mut adapter_changed = proxy.receive_adapter_changed().await;
    let mut legacy_pairing_changed = proxy.receive_legacy_pairing_changed().await;
    let mut modalias_changed = proxy.receive_modalias_changed().await;
    let mut rssi_changed = proxy.receive_rssi_changed().await;
    let mut tx_power_changed = proxy.receive_tx_power_changed().await;
    let mut manufacturer_data_changed = proxy.receive_manufacturer_data_changed().await;
    let mut service_data_changed = proxy.receive_service_data_changed().await;
    let mut services_resolved_changed = proxy.receive_services_resolved_changed().await;
    let mut advertising_flags_changed = proxy.receive_advertising_flags_changed().await;
    let mut advertising_data_changed = proxy.receive_advertising_data_changed().await;
    let mut preferred_bearer_changed = proxy.receive_preferred_bearer_changed().await;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                debug!("Device monitoring cancelled for {}", device.object_path);
                return;
            }
            Some(change) = address_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.address.set(value);
                }
            }
            Some(change) = address_type_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.address_type.set(AddressType::from(value.as_str()));
                }
            }
            Some(change) = name_changed.next() => {
                let new_name = change.get().await.ok();
                device.name.set(new_name);
            }
            Some(change) = icon_changed.next() => {
                let new_icon = change.get().await.ok();
                device.icon.set(new_icon);
            }
            Some(change) = class_changed.next() => {
                let new_class = change.get().await.ok();
                device.class.set(new_class);
            }
            Some(change) = appearance_changed.next() => {
                let new_appearance = change.get().await.ok();
                device.appearance.set(new_appearance);
            }
            Some(change) = uuids_changed.next() => {
                let new_uuids = change.get().await.ok();
                device.uuids.set(new_uuids);
            }
            Some(change) = paired_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.paired.set(value);
                }
            }
            Some(change) = bonded_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.bonded.set(value);
                }
            }
            Some(change) = connected_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.connected.set(value);
                    let _ = notifier_tx.send(ServiceNotification::DeviceConnectionChanged);
                }
            }
            Some(change) = trusted_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.trusted.set(value);
                }
            }
            Some(change) = blocked_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.blocked.set(value);
                }
            }
            Some(change) = wake_allowed_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.wake_allowed.set(value);
                }
            }
            Some(change) = alias_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.alias.set(value);
                }
            }
            Some(change) = adapter_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.adapter.set(value);
                }
            }
            Some(change) = legacy_pairing_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.legacy_pairing.set(value);
                }
            }
            Some(change) = modalias_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.modalias.set(if value.is_empty() { None } else { Some(value) });
                }
            }
            Some(change) = rssi_changed.next() => {
                let new_rssi = change.get().await.ok();
                device.rssi.set(new_rssi);
            }
            Some(change) = tx_power_changed.next() => {
                let new_tx_power = change.get().await.ok();
                device.tx_power.set(new_tx_power);
            }
            Some(change) = manufacturer_data_changed.next() => {
                let new_manufacturer_data = change.get().await.ok();
                device.manufacturer_data.set(new_manufacturer_data);
            }
            Some(change) = service_data_changed.next() => {
                let new_service_data = change.get().await.ok();
                device.service_data.set(new_service_data);
            }
            Some(change) = services_resolved_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.services_resolved.set(value);
                }
            }
            Some(change) = advertising_flags_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.advertising_flags.set(value);
                }
            }
            Some(change) = advertising_data_changed.next() => {
                if let Ok(value) = change.get().await {
                    device.advertising_data.set(value);
                }
            }
            Some(change) = preferred_bearer_changed.next() => {
                match change.get().await {
                    Ok(new_preferred_bearer) => {
                        device.preferred_bearer.set(
                            Some(PreferredBearer::from(new_preferred_bearer.as_str()))
                        );
                    }
                    Err(_) => {
                        device.preferred_bearer.set(None);
                    }
                }
            }
            else => {
                debug!("All property streams ended for device {}", device.object_path);
                break;
            }
        }
    }

    debug!(
        "Property monitoring ended for device {}",
        device.object_path
    );
}
