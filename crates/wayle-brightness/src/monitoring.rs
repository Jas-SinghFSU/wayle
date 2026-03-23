use std::{collections::HashMap, sync::Arc};

use tokio::sync::broadcast::error::RecvError;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};
use wayle_core::Property;
use wayle_traits::{ModelMonitoring, ServiceMonitoring};

use crate::{
    Error,
    backend::types::{BrightnessEvent, CommandSender, EventSender},
    core::BacklightDevice,
    service::BrightnessService,
    types::{BacklightInfo, DeviceName},
};

type DeviceMap = HashMap<DeviceName, Arc<BacklightDevice>>;

impl ServiceMonitoring for BrightnessService {
    type Error = Error;

    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let mut event_rx = self.event_tx.subscribe();

        let command_tx = self.command_tx.clone();
        let event_tx = self.event_tx.clone();
        let devices = self.devices.clone();
        let primary = self.primary.clone();
        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            let mut devices_map: DeviceMap = HashMap::new();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("brightness monitoring stopped");
                        return;
                    }

                    result = event_rx.recv() => {
                        match result {
                            Ok(event) => handle_event(
                                event,
                                &mut devices_map,
                                &command_tx,
                                &event_tx,
                                &cancellation_token,
                                &devices,
                                &primary,
                            ).await,

                            Err(RecvError::Lagged(count)) => {
                                warn!(
                                    count,
                                    "brightness events lost, state may be stale"
                                );
                            }

                            Err(RecvError::Closed) => return,
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

async fn handle_event(
    event: BrightnessEvent,
    devices_map: &mut DeviceMap,
    command_tx: &CommandSender,
    event_tx: &EventSender,
    parent_token: &CancellationToken,
    devices: &Property<Vec<Arc<BacklightDevice>>>,
    primary: &Property<Option<Arc<BacklightDevice>>>,
) {
    match event {
        BrightnessEvent::DeviceAdded(info) => {
            upsert_device(
                info,
                devices_map,
                command_tx,
                event_tx,
                parent_token,
                devices,
                primary,
            )
            .await;
        }

        BrightnessEvent::DeviceChanged(info) => {
            if let Some(existing) = devices_map.get(&info.name) {
                existing.update_brightness(info.brightness);
                return;
            }

            upsert_device(
                info,
                devices_map,
                command_tx,
                event_tx,
                parent_token,
                devices,
                primary,
            )
            .await;
        }

        BrightnessEvent::DeviceRemoved(name) => {
            let key = DeviceName::new(name);
            cancel_existing(devices_map, &key);
            devices_map.remove(&key);
            update_properties(devices_map, devices, primary);
        }
    }
}

async fn upsert_device(
    info: BacklightInfo,
    devices_map: &mut DeviceMap,
    command_tx: &CommandSender,
    event_tx: &EventSender,
    parent_token: &CancellationToken,
    devices: &Property<Vec<Arc<BacklightDevice>>>,
    primary: &Property<Option<Arc<BacklightDevice>>>,
) {
    cancel_existing(devices_map, &info.name);

    let device = create_device(&info, command_tx, event_tx, parent_token);

    if let Err(err) = device.clone().start_monitoring().await {
        warn!(error = %err, "cannot start device monitoring");
    }

    devices_map.insert(info.name, device);
    update_properties(devices_map, devices, primary);
}

fn create_device(
    info: &BacklightInfo,
    command_tx: &CommandSender,
    event_tx: &EventSender,
    parent_token: &CancellationToken,
) -> Arc<BacklightDevice> {
    Arc::new(BacklightDevice::from_info(
        info,
        command_tx.clone(),
        Some(event_tx.clone()),
        Some(parent_token.child_token()),
    ))
}

fn cancel_existing(devices_map: &DeviceMap, name: &DeviceName) {
    let token = devices_map
        .get(name)
        .and_then(|device| device.cancellation_token.as_ref());

    if let Some(token) = token {
        token.cancel();
    }
}

fn update_properties(
    devices_map: &DeviceMap,
    devices: &Property<Vec<Arc<BacklightDevice>>>,
    primary: &Property<Option<Arc<BacklightDevice>>>,
) {
    let device_list: Vec<_> = devices_map.values().cloned().collect();
    devices.set(device_list);

    let best = devices_map
        .values()
        .max_by_key(|device| device.backlight_type)
        .cloned();

    primary.set(best);
}
