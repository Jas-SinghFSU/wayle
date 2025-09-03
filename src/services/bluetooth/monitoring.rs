use std::sync::Arc;

use futures::StreamExt;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing::debug;
use zbus::{Connection, fdo::ObjectManagerProxy, zvariant::OwnedObjectPath};

use super::{
    BluetoothError,
    core::{Adapter, Device, LiveAdapterParams, LiveDeviceParams},
    service::BluetoothService,
    types::{ADAPTER_INTERFACE, DEVICE_INTERFACE, ServiceNotification},
};
use crate::{
    remove_and_cancel,
    services::{
        bluetooth::types::BLUEZ_SERVICE,
        common::{Property, ROOT_PATH, property::PropertyStream},
        traits::{Reactive, ServiceMonitoring},
    },
};

impl ServiceMonitoring for BluetoothService {
    type Error = BluetoothError;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let object_manager =
            ObjectManagerProxy::new(&self.zbus_connection, BLUEZ_SERVICE, ROOT_PATH).await?;

        monitor_devices(
            &self.zbus_connection,
            &object_manager,
            self.cancellation_token.child_token(),
            &self.devices,
            &self.notifier_tx,
        )
        .await?;
        monitor_adapters(
            &self.zbus_connection,
            &object_manager,
            self.cancellation_token.child_token(),
            &self.adapters,
        )
        .await?;
        monitor_primary_adapter(
            &self.primary_adapter,
            &self.adapters,
            self.cancellation_token.clone(),
        )
        .await?;
        monitor_available(
            &self.available,
            &self.primary_adapter,
            self.cancellation_token.clone(),
        )
        .await?;
        monitor_enabled(
            &self.enabled,
            &self.primary_adapter,
            self.cancellation_token.clone(),
        )
        .await?;
        monitor_connected(
            &self.connected,
            &self.devices,
            self.notifier_tx.subscribe(),
            self.cancellation_token.clone(),
        )
        .await?;

        Ok(())
    }
}
async fn monitor_devices(
    connection: &Connection,
    object_manager: &ObjectManagerProxy<'_>,
    cancellation_token: CancellationToken,
    devices: &Property<Vec<Arc<Device>>>,
    notifier_tx: &broadcast::Sender<ServiceNotification>,
) -> Result<(), BluetoothError> {
    let mut device_interface_added = object_manager
        .receive_interfaces_added_with_args(&[(1, DEVICE_INTERFACE)])
        .await?;
    let mut device_interface_removed = object_manager
        .receive_interfaces_removed_with_args(&[(1, DEVICE_INTERFACE)])
        .await?;
    let devices_prop = devices.clone();
    let connection = connection.clone();
    let notifier_tx = notifier_tx.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Bluetooth 'devices' monitoring cancelled");
                    return;
                }
                Some(added) = device_interface_added.next() => {
                    if let Ok(args) = added.args() {
                        let object_path: OwnedObjectPath = args.object_path.into();

                        handle_device_added(
                            &connection,
                            cancellation_token.child_token(),
                            &devices_prop,
                            object_path,
                            &notifier_tx,
                        )
                        .await;
                    }
                }
                Some(removed) = device_interface_removed.next() => {
                    if let Ok(args) = removed.args() {
                        let object_path: OwnedObjectPath = args.object_path.into();
                        remove_and_cancel!(devices_prop, object_path);
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
                    debug!("Bluetooth 'adapter' monitoring cancelled");
                    return;
                }
                Some(added) = adapter_interface_added.next() => {
                    if let Ok(args) = added.args() {
                        let object_path: OwnedObjectPath = args.object_path.into();

                        handle_adapter_added(
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
                        remove_and_cancel!(adapters_prop, object_path);
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
    cancellation_token: CancellationToken,
) -> Result<(), BluetoothError> {
    let primary_adapter_prop = primary_adapter.clone();
    let adapters_prop = adapters.clone();

    tokio::spawn(async move {
        let mut adapters_stream = adapters_prop.watch();
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Bluetooth 'primary_adapter' monitoring cancelled");
                    return;
                }
                Some(adapters) = adapters_stream.next() => {
                    let current_primary = primary_adapter_prop.get();

                    let new_primary = select_primary_adapter(current_primary, &adapters);
                    primary_adapter_prop.set(new_primary);
                }
            }
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
        return find_best_adapter(adapters);
    };

    if !adapters
        .iter()
        .any(|a| a.object_path == current.object_path)
    {
        return find_best_adapter(adapters);
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
    cancellation_token: CancellationToken,
) -> Result<(), BluetoothError> {
    let enabled_prop = enabled.clone();
    let primary_adapter_prop = primary_adapter.clone();

    tokio::spawn(async move {
        let mut primary_stream = primary_adapter_prop.watch();
        let mut current_powered_stream: Option<PropertyStream<bool>> = None;

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Bluetooth 'enabled' monitor cancelled");
                    return;
                }
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
    cancellation_token: CancellationToken,
) -> Result<(), BluetoothError> {
    let available_prop = available.clone();
    let primary_adapter_prop = primary_adapter.clone();

    tokio::spawn(async move {
        let mut primary_stream = primary_adapter_prop.watch();

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Bluetooth 'available' monitor cancelled");
                    return;
                }
                Some(primary) = primary_stream.next() => {
                    available_prop.set(primary.is_some());
                }
            }
        }
    });

    Ok(())
}

async fn monitor_connected(
    connected: &Property<Vec<String>>,
    devices: &Property<Vec<Arc<Device>>>,
    mut notifier_rx: broadcast::Receiver<ServiceNotification>,
    cancellation_token: CancellationToken,
) -> Result<(), BluetoothError> {
    let devices_prop = devices.clone();
    let connected_prop = connected.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    debug!("Bluetooth 'connected' monitor cancelled");
                    return;
                }
                Ok(notif) = notifier_rx.recv() => {
                    match notif {
                        ServiceNotification::DeviceConnectionChanged => {
                            handle_device_connection_changed(devices_prop.clone(), connected_prop.clone())
                            .await;
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

async fn handle_device_added(
    connection: &Connection,
    cancellation_token: CancellationToken,
    devices: &Property<Vec<Arc<Device>>>,
    object_path: OwnedObjectPath,
    notifier_tx: &broadcast::Sender<ServiceNotification>,
) {
    let mut device_list = devices.get();
    if !device_list
        .iter()
        .any(|device| device.object_path == object_path)
        && let Ok(created_device) = Device::get_live(LiveDeviceParams {
            connection,
            path: object_path,
            cancellation_token: cancellation_token.child_token(),
            notifier_tx,
        })
        .await
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
        && let Ok(created_adapter) = Adapter::get_live(LiveAdapterParams {
            connection,
            path: object_path,
            cancellation_token: cancellation_token.child_token(),
        })
        .await
    {
        adapters_list.push(created_adapter);
        adapters.set(adapters_list);
    }
}

async fn handle_device_connection_changed(
    devices: Property<Vec<Arc<Device>>>,
    connected: Property<Vec<String>>,
) {
    let connected_devices = devices
        .get()
        .iter()
        .filter_map(|device| {
            if device.connected.get() {
                Some(device.address.get())
            } else {
                None
            }
        })
        .collect();

    connected.set(connected_devices);
}
