use std::sync::Arc;

use futures::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, fdo::ObjectManagerProxy, zvariant::OwnedObjectPath};

use super::{
    BluetoothError,
    core::{Adapter, Device},
    types::{ADAPTER_INTERFACE, DEVICE_INTERFACE},
};
use crate::services::{
    bluetooth::types::BLUEZ_SERVICE,
    common::{Property, ROOT_PATH, property::PropertyStream},
};

pub(crate) struct BluetoothMonitoring;

impl BluetoothMonitoring {
    pub(crate) async fn start(
        connection: &Connection,
        cancellation_token: CancellationToken,
        adapters: &Property<Vec<Arc<Adapter>>>,
        primary_adapter: &Property<Option<Arc<Adapter>>>,
        devices: &Property<Vec<Arc<Device>>>,
        enabled: &Property<bool>,
        available: &Property<bool>,
    ) -> Result<(), BluetoothError> {
        let object_manager = ObjectManagerProxy::new(connection, BLUEZ_SERVICE, ROOT_PATH).await?;

        Self::monitor_devices(
            connection,
            &object_manager,
            cancellation_token.child_token(),
            devices,
        )
        .await?;
        Self::monitor_adapters(
            connection,
            &object_manager,
            cancellation_token.child_token(),
            adapters,
        )
        .await?;
        Self::monitor_primary_adapter(primary_adapter, adapters).await?;
        Self::monitor_available(available, primary_adapter).await?;
        Self::monitor_enabled(enabled, primary_adapter).await?;

        Ok(())
    }

    async fn monitor_devices(
        connection: &Connection,
        object_manager: &ObjectManagerProxy<'_>,
        cancellation_token: CancellationToken,
        devices: &Property<Vec<Arc<Device>>>,
    ) -> Result<(), BluetoothError> {
        let mut device_interface_added = object_manager
            .receive_interfaces_added_with_args(&[(1, DEVICE_INTERFACE)])
            .await?;
        let mut device_interface_removed = object_manager
            .receive_interfaces_removed_with_args(&[(1, DEVICE_INTERFACE)])
            .await?;
        let devices_prop = devices.clone();
        let connection = connection.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        debug!("Bluetooth Device monitoring cancelled");
                        return;
                    }
                    Some(added) = device_interface_added.next() => {
                        if let Ok(args) = added.args() {
                            let object_path: OwnedObjectPath = args.object_path.into();

                            Self::handle_device_added(
                                &connection,
                                cancellation_token.child_token(),
                                &devices_prop,
                                object_path,
                            )
                            .await;
                        }
                    }
                    Some(removed) = device_interface_removed.next() => {
                        if let Ok(args) = removed.args() {
                            let object_path: OwnedObjectPath = args.object_path.into();
                            let mut device_list = devices_prop.get();

                            device_list.retain(|device| device.object_path != object_path);
                            devices_prop.set(device_list);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn monitor_adapters(
        connection: &Connection,
        object_manager: &ObjectManagerProxy<'_>,
        cancellation_token: CancellationToken,
        adapters: &Property<Vec<Arc<Adapter>>>,
    ) -> Result<(), BluetoothError> {
        let mut adapter_interface_added = object_manager
            .receive_interfaces_added_with_args(&[(1, ADAPTER_INTERFACE)])
            .await?;
        let mut adapter_interface_removed = object_manager
            .receive_interfaces_removed_with_args(&[(1, ADAPTER_INTERFACE)])
            .await?;

        let adapters_prop = adapters.clone();
        let connection = connection.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        debug!("Bluetooth Adapter monitoring cancelled");
                        return;
                    }
                    Some(added) = adapter_interface_added.next() => {
                        if let Ok(args) = added.args() {
                            let object_path: OwnedObjectPath = args.object_path.into();

                            Self::handle_adapter_added(
                                &connection,
                                cancellation_token.child_token(),
                                &adapters_prop,
                                object_path,
                            )
                            .await;
                        }
                    }
                    Some(removed) = adapter_interface_removed.next() => {
                        if let Ok(args) = removed.args() {
                            let object_path: OwnedObjectPath = args.object_path.into();
                            let mut adapters_list = adapters_prop.get();

                            adapters_list.retain(|adapter| adapter.object_path != object_path);
                            adapters_prop.set(adapters_list);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn monitor_primary_adapter(
        primary_adapter: &Property<Option<Arc<Adapter>>>,
        adapters: &Property<Vec<Arc<Adapter>>>,
    ) -> Result<(), BluetoothError> {
        let primary_adapter_prop = primary_adapter.clone();
        let adapters_prop = adapters.clone();

        tokio::spawn(async move {
            let mut adapters_stream = adapters_prop.watch();
            while (adapters_stream.next().await).is_some() {
                let current_primary = primary_adapter_prop.get();
                let adapters_list = adapters_prop.get();

                let new_primary = Self::select_primary_adapter(current_primary, &adapters_list);
                primary_adapter_prop.set(new_primary);
            }
        });

        Ok(())
    }

    fn select_primary_adapter(
        current: Option<Arc<Adapter>>,
        adapters: &[Arc<Adapter>],
    ) -> Option<Arc<Adapter>> {
        if adapters.is_empty() {
            return None;
        }

        let Some(current) = current else {
            return Self::find_best_adapter(adapters);
        };

        if !adapters
            .iter()
            .any(|a| a.object_path == current.object_path)
        {
            return Self::find_best_adapter(adapters);
        }

        if current.powered.get() {
            return Some(current);
        }

        adapters
            .iter()
            .find(|a| a.powered.get())
            .cloned()
            .or(Some(current))
    }

    fn find_best_adapter(adapters: &[Arc<Adapter>]) -> Option<Arc<Adapter>> {
        adapters
            .iter()
            .find(|a| a.powered.get())
            .or_else(|| adapters.first())
            .cloned()
    }

    async fn monitor_enabled(
        enabled: &Property<bool>,
        primary_adapter: &Property<Option<Arc<Adapter>>>,
    ) -> Result<(), BluetoothError> {
        let enabled_prop = enabled.clone();
        let primary_adapter_prop = primary_adapter.clone();

        tokio::spawn(async move {
            let mut primary_stream = primary_adapter_prop.watch();
            let mut current_powered_stream: Option<PropertyStream<bool>> = None;

            loop {
                tokio::select! {
                    Some(primary) = primary_stream.next() => {
                        current_powered_stream = primary
                            .as_ref()
                            .map(|a| Box::new(a.powered.watch()) as PropertyStream<bool>);
                        enabled_prop.set(primary.as_ref().is_some_and(|a| a.powered.get()));
                    }
                    Some(powered) = async {
                        match &mut current_powered_stream {
                            Some(stream) => stream.next().await,
                            None => std::future::pending().await
                        }
                    } => {
                        enabled_prop.set(powered);
                    }
                }
            }
        });

        Ok(())
    }

    async fn monitor_available(
        available: &Property<bool>,
        primary_adapter: &Property<Option<Arc<Adapter>>>,
    ) -> Result<(), BluetoothError> {
        let available_prop = available.clone();
        let primary_adapter_prop = primary_adapter.clone();

        tokio::spawn(async move {
            let mut primary_stream = primary_adapter_prop.watch();
            while let Some(primary) = primary_stream.next().await {
                available_prop.set(primary.is_some());
            }
        });

        Ok(())
    }

    async fn handle_device_added(
        connection: &Connection,
        cancellation_token: CancellationToken,
        devices: &Property<Vec<Arc<Device>>>,
        object_path: OwnedObjectPath,
    ) {
        let mut device_list = devices.get();
        if !device_list
            .iter()
            .any(|device| device.object_path == object_path)
            && let Ok(created_device) =
                Device::get_live(connection, object_path, cancellation_token.child_token()).await
            {
                device_list.push(created_device);
                devices.set(device_list);
            }
    }

    async fn handle_adapter_added(
        connection: &Connection,
        cancellation_token: CancellationToken,
        adapters: &Property<Vec<Arc<Adapter>>>,
        object_path: OwnedObjectPath,
    ) {
        let mut adapters_list = adapters.get();
        if !adapters_list
            .iter()
            .any(|adapter| adapter.object_path == object_path)
            && let Ok(created_adapter) =
                Adapter::get_live(connection, object_path, cancellation_token.child_token()).await
            {
                adapters_list.push(created_adapter);
                adapters.set(adapters_list);
            }
    }
}
