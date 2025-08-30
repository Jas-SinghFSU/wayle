use std::{collections::HashMap, sync::Arc};

use tokio_util::sync::CancellationToken;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use super::{
    core::{Adapter, Device},
    error::BluetoothError,
    types::{ADAPTER_INTERFACE, DEVICE_INTERFACE},
};
use crate::services::{common::ObjectManagerProxy, network::connection};

pub(crate) struct BluetoothDiscovery {
    pub adapters: Vec<Arc<Adapter>>,
    pub primary_adapter: Option<Arc<Adapter>>,
    pub devices: Vec<Arc<Device>>,
    pub available: bool,
    pub enabled: bool,
    pub connected: bool,
}

impl BluetoothDiscovery {
    pub(crate) async fn new(
        connection: &Connection,
        cancellation_token: CancellationToken,
    ) -> Result<Self, BluetoothError> {
        let object_manager = ObjectManagerProxy::builder(connection)
            .destination("org.bluez")?
            .path("/")?
            .build()
            .await?;
        let managed_objects = object_manager.get_managed_objects().await?;

        let mut adapters = Vec::new();
        let mut devices = Vec::new();

        for (object_path, interfaces) in managed_objects {
            Self::extract_adapter(
                adapters,
                connection,
                cancellation_token,
                object_path,
                interfaces,
            );
            Self::extract_device(
                devices,
                connection,
                cancellation_token,
                object_path,
                interfaces,
            );
        }

        let primary_adapter = adapters
            .iter()
            .find(|adapter| adapter.powered.get())
            .or_else(|| adapters.first())
            .cloned();

        let enabled = primary_adapter
            .as_ref()
            .map_or(false, |adapter| adapter.powered.get());

        Ok(Self {
            adapters,
            primary_adapter,
            available: primary_adapter.is_some(),
            enabled,
        })
    }

    pub async fn extract_adapter(
        mut adapters: Vec<Arc<Adapter>>,
        connection: &Connection,
        cancellation_token: CancellationToken,
        object_path: OwnedObjectPath,
        interfaces: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> () {
        if !interfaces.contains_key(ADAPTER_INTERFACE) {
            return;
        }

        match Adapter::get_live(connection, object_path.clone(), cancellation_token.clone()).await {
            Ok(adapter) => adapters.push(adapter),
            Err(e) => {
                tracing::warn!(
                    "Failed to create adapter for path {}: {}",
                    object_path.to_string(),
                    e
                );
            }
        }
    }

    pub async fn extract_device(
        mut devices: Vec<Arc<Device>>,
        connection: &Connection,
        cancellation_token: CancellationToken,
        object_path: OwnedObjectPath,
        interfaces: HashMap<String, HashMap<String, OwnedValue>>,
    ) -> () {
        if !interfaces.contains_key(DEVICE_INTERFACE) {
            return;
        }

        match Device::get_live(connection, object_path.clone(), cancellation_token.clone()).await {
            Ok(device) => devices.push(device),
            Err(e) => {
                tracing::warn!(
                    "Failed to create device for path {}: {}",
                    object_path.to_string(),
                    e
                );
            }
        }
    }
}
