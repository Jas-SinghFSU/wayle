use std::sync::Arc;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, warn};
use wayle_common::Property;
use wayle_traits::{Reactive, ServiceMonitoring};
use zbus::{Connection, fdo::DBusProxy};

use super::{
    core::item::{LiveTrayItemParams, TrayItem},
    error::Error,
    events::TrayEvent,
    proxy::status_notifier_watcher::StatusNotifierWatcherProxy,
    service::SystemTrayService,
};

impl ServiceMonitoring for SystemTrayService {
    type Error = Error;

    #[instrument(skip_all, err)]
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        if self.is_watcher {
            handle_watcher_mode(self).await?;
        } else {
            handle_host_mode(self).await?;
        }
        Ok(())
    }
}

#[instrument(skip_all)]
async fn handle_watcher_mode(service: &SystemTrayService) -> Result<(), Error> {
    let mut event_receiver = service.event_tx.subscribe();
    let items = service.items.clone();
    let cancellation_token = service.cancellation_token.clone();
    let connection = service.connection.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    info!("Systray watcher monitoring cancelled");
                    return;
                }
                Ok(event) = event_receiver.recv() => {
                    match event {
                        TrayEvent::ItemRegistered(bus_name) => {
                            if let Err(e) = handle_item_registered(
                                &bus_name,
                                &items,
                                &connection,
                                &cancellation_token,
                            )
                            .await
                            {
                                warn!("Failed to handle registration for {}: {}", bus_name, e);
                            }
                        }
                        TrayEvent::ItemUnregistered(bus_name) => {
                            handle_item_unregistered(&bus_name, &items);
                        }
                        TrayEvent::ServiceDisconnected(bus_name) => {
                            handle_item_unregistered(&bus_name, &items);
                        }
                    }
                }
            }
        }
    });

    monitor_name_owner_changes(service).await?;
    Ok(())
}

#[instrument(skip_all)]
async fn handle_host_mode(service: &SystemTrayService) -> Result<(), Error> {
    let watcher = StatusNotifierWatcherProxy::new(&service.connection).await?;

    let mut registered = watcher.receive_status_notifier_item_registered().await?;
    let mut unregistered = watcher.receive_status_notifier_item_unregistered().await?;

    let items = service.items.clone();
    let connection = service.connection.clone();
    let cancellation_token = service.cancellation_token.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    info!("Systray host monitoring cancelled");
                    return;
                }
                Some(signal) = registered.next() => {
                    if let Ok(args) = signal.args()
                        && let Err(e) = handle_item_registered(
                            &args.service,
                            &items,
                            &connection,
                            &cancellation_token,
                        )
                        .await
                        {
                            warn!("Failed to handle registration for {}: {}", args.service, e);
                        }
                }
                Some(signal) = unregistered.next() => {
                    if let Ok(args) = signal.args() {
                        handle_item_unregistered(&args.service, &items);
                    }
                }
            }
        }
    });

    Ok(())
}

#[instrument(
    skip(connection, items, cancellation_token),
    fields(bus_name = %bus_name),
    err
)]
async fn handle_item_registered(
    bus_name: &str,
    items: &Property<Vec<Arc<TrayItem>>>,
    connection: &Connection,
    cancellation_token: &CancellationToken,
) -> Result<(), Error> {
    let params = LiveTrayItemParams {
        connection,
        service: bus_name.to_string(),
        cancellation_token,
    };
    let item = TrayItem::get_live(params).await?;

    let mut list = items.get();
    list.retain(|existing| existing.bus_name.get() != bus_name);
    list.push(item);
    items.set(list);

    debug!("Item registered: {}", bus_name);
    Ok(())
}

#[instrument(skip(items), fields(bus_name = %bus_name))]
fn handle_item_unregistered(bus_name: &str, items: &Property<Vec<Arc<TrayItem>>>) {
    let mut list = items.get();
    list.retain(|item| {
        if item.bus_name.get() != bus_name {
            return true;
        }
        if let Some(token) = item.cancellation_token.as_ref() {
            token.cancel();
        }
        false
    });
    items.set(list);

    debug!("Item unregistered: {}", bus_name);
}

#[instrument(skip(service), err)]
async fn monitor_name_owner_changes(service: &SystemTrayService) -> Result<(), Error> {
    let Ok(dbus_proxy) = DBusProxy::new(&service.connection).await else {
        warn!("Failed to create DBus proxy for name monitoring");
        return Ok(());
    };

    let Ok(mut name_owner_changed) = dbus_proxy.receive_name_owner_changed().await else {
        warn!("Failed to subscribe to NameOwnerChanged");
        return Ok(());
    };

    let event_tx = service.event_tx.clone();
    let items = service.items.clone();
    let cancellation_token = service.cancellation_token.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    return;
                }
                Some(signal) = name_owner_changed.next() => {
                    let Ok(args) = signal.args() else {
                        continue;
                    };

                    if args.new_owner.is_some() {
                        continue;
                    }

                    let list = items.get();
                    if !list.iter().any(|item| item.bus_name.get() == args.name.to_string()) {
                        continue;
                    }

                    let _ = event_tx.send(TrayEvent::ServiceDisconnected(args.name.to_string()));
                }
            }
        }
    });

    Ok(())
}
