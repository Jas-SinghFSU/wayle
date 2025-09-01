use std::{collections::HashMap, sync::Arc};

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use zbus::{
    Connection,
    fdo::ObjectManagerProxy,
    names::OwnedInterfaceName,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use super::{
    core::{Adapter, Device, LiveAdapterParams, LiveDeviceParams},
    error::BluetoothError,
    types::{ADAPTER_INTERFACE, BLUEZ_SERVICE, DEVICE_INTERFACE, ServiceNotification},
};
use crate::services::{common::ROOT_PATH, traits::Reactive};
pub(crate) struct BluetoothDiscovery {
    pub adapters: Vec<Arc<Adapter>>,
    pub primary_adapter: Option<Arc<Adapter>>,
    pub devices: Vec<Arc<Device>>,
    pub available: bool,
    pub enabled: bool,
    pub connected: Vec<String>,
}

impl BluetoothDiscovery {
    pub(crate) async fn new(
        connection: &Connection,
        cancellation_token: CancellationToken,
        notifier_tx: &broadcast::Sender<ServiceNotification>,
    ) -> Result<Self, BluetoothError> {
        let object_manager = ObjectManagerProxy::new(connection, BLUEZ_SERVICE, ROOT_PATH).await?;
        let managed_objects = object_manager.get_managed_objects().await.map_err(|e| {
            BluetoothError::OperationFailed {
                operation: "object_manager.get_managed_objects",
                reason: e.to_string(),
            }
        })?;

        let mut adapters = Vec::new();
        let mut devices = Vec::new();

        for (object_path, interfaces) in managed_objects {
            Self::extract_adapter(
                &mut adapters,
                connection,
                cancellation_token.child_token(),
                object_path.clone(),
                interfaces.clone(),
            )
            .await;
            Self::extract_device(
                &mut devices,
                connection,
                cancellation_token.child_token(),
                object_path,
                interfaces,
                notifier_tx,
            )
            .await;
        }

        let primary_adapter = adapters
            .iter()
            .find(|adapter| adapter.powered.get())
            .or_else(|| adapters.first())
            .cloned();
        let available = primary_adapter.as_ref().is_some();
        let enabled = primary_adapter
            .as_ref()
            .is_some_and(|adapter| adapter.powered.get());
        let connected = devices
            .iter()
            .filter_map(|device| {
                if device.connected.get() {
                    Some(device.address.get())
                } else {
                    None
                }
            })
            .collect();

        Ok(Self {
            adapters,
            devices,
            primary_adapter,
            available,
            enabled,
            connected,
        })
    }

    async fn extract_adapter(
        adapters: &mut Vec<Arc<Adapter>>,
        connection: &Connection,
        cancellation_token: CancellationToken,
        object_path: OwnedObjectPath,
        interfaces: HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>,
    ) -> () {
        if !interfaces.contains_key(ADAPTER_INTERFACE) {
            return;
        }

        match Adapter::get_live(LiveAdapterParams {
            connection,
            path: object_path.clone(),
            cancellation_token,
        })
        .await
        {
            Ok(adapter) => adapters.push(adapter),
            Err(e) => {
                warn!(
                    "Failed to create adapter for path {}: {}",
                    object_path.to_string(),
                    e
                );
            }
        }
    }

    async fn extract_device(
        devices: &mut Vec<Arc<Device>>,
        connection: &Connection,
        cancellation_token: CancellationToken,
        object_path: OwnedObjectPath,
        interfaces: HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>,
        notifier_tx: &broadcast::Sender<ServiceNotification>,
    ) -> () {
        if !interfaces.contains_key(DEVICE_INTERFACE) {
            return;
        }

        match Device::get_live(LiveDeviceParams {
            connection,
            path: object_path.clone(),
            cancellation_token,
            notifier_tx,
        })
        .await
        {
            Ok(device) => devices.push(device),
            Err(e) => {
                warn!(
                    "Failed to create device for path {}: {}",
                    object_path.to_string(),
                    e
                );
            }
        }
    }
}
