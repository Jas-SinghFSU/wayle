use std::sync::Arc;

use libpulse_binding::{
    callbacks::ListResult,
    context::{Context, subscribe::Facility},
    volume::ChannelVolumes,
};

use crate::services::audio::{
    backend::{
        conversion::{create_device_info_from_sink, create_device_info_from_source},
        types::{DeviceStore, EventSender},
    },
    events::AudioEvent,
    types::device::{Device, DeviceKey, DeviceType},
};

pub(crate) fn trigger_discovery(context: &Context, devices: &DeviceStore, events_tx: &EventSender) {
    let introspect = context.introspect();

    let devices_clone = Arc::clone(devices);
    let events_tx_clone = events_tx.clone();

    introspect.get_sink_info_list(move |sink_list| {
        if let ListResult::Item(sink) = sink_list {
            let sink_info = create_device_info_from_sink(sink);
            let device_key = sink_info.key();
            let device_data = Device::Sink(sink_info);

            process_device_update(device_key, device_data, &devices_clone, &events_tx_clone);
        }
    });

    let devices_clone = Arc::clone(devices);
    let events_tx_clone = events_tx.clone();

    introspect.get_source_info_list(move |source_list| {
        if let ListResult::Item(source) = source_list {
            let source_info = create_device_info_from_source(source);
            let device_key = source_info.key();
            let device_data = Device::Source(source_info);

            process_device_update(device_key, device_data, &devices_clone, &events_tx_clone);
        }
    });
}

pub(crate) fn trigger_refresh(
    context: &Context,
    devices: &DeviceStore,
    events_tx: &EventSender,
    device_key: DeviceKey,
    facility: Facility,
) {
    let introspect = context.introspect();

    let devices_clone = Arc::clone(devices);
    let events_tx_clone = events_tx.clone();

    match facility {
        Facility::Sink => {
            introspect.get_sink_info_by_index(device_key.index, move |sink_list| {
                if let ListResult::Item(sink) = sink_list {
                    let sink_info = create_device_info_from_sink(sink);
                    let device_data = Device::Sink(sink_info);

                    process_device_update(
                        device_key,
                        device_data,
                        &devices_clone,
                        &events_tx_clone,
                    );
                }
            });
        }
        Facility::Source => {
            introspect.get_source_info_by_index(device_key.index, move |source_list| {
                if let ListResult::Item(source) = source_list {
                    let source_info = create_device_info_from_source(source);
                    let device_data = Device::Source(source_info);

                    process_device_update(
                        device_key,
                        device_data,
                        &devices_clone,
                        &events_tx_clone,
                    );
                }
            });
        }
        _ => {}
    }
}

pub(crate) fn process_device_update(
    device_key: DeviceKey,
    device_data: Device,
    devices: &DeviceStore,
    events_tx: &EventSender,
) {
    let Ok(mut devices_guard) = devices.write() else {
        return;
    };

    let is_new = !devices_guard.contains_key(&device_key);
    devices_guard.insert(device_key, device_data.clone());

    let event = if is_new {
        AudioEvent::DeviceAdded(device_data)
    } else {
        AudioEvent::DeviceChanged(device_data)
    };

    let _ = events_tx.send(event);
}

pub(crate) fn set_device_volume(
    context: &Context,
    device_key: DeviceKey,
    volume: ChannelVolumes,
    devices: &DeviceStore,
) {
    let devices_clone = Arc::clone(devices);
    let mut introspect = context.introspect();

    let device_info = {
        if let Ok(devices_guard) = devices_clone.read() {
            devices_guard
                .values()
                .find(|d| d.key() == device_key)
                .cloned()
        } else {
            return;
        }
    };

    if let Some(info) = device_info {
        match info.key().device_type {
            DeviceType::Output => {
                introspect.set_sink_volume_by_index(device_key.index, &volume, None);
            }
            DeviceType::Input => {
                introspect.set_source_volume_by_index(device_key.index, &volume, None);
            }
        }
    }
}

pub(crate) fn set_device_mute(
    context: &Context,
    device_key: DeviceKey,
    muted: bool,
    devices: &DeviceStore,
) {
    let devices_clone = Arc::clone(devices);
    let mut introspect = context.introspect();

    let device_info = {
        if let Ok(devices_guard) = devices_clone.read() {
            devices_guard
                .values()
                .find(|d| d.key() == device_key)
                .cloned()
        } else {
            return;
        }
    };

    if let Some(info) = device_info {
        match info.key().device_type {
            DeviceType::Output => {
                introspect.set_sink_mute_by_index(device_key.index, muted, None);
            }
            DeviceType::Input => {
                introspect.set_source_mute_by_index(device_key.index, muted, None);
            }
        }
    }
}

pub(crate) fn set_device_port(
    context: &Context,
    device_key: DeviceKey,
    port: String,
    devices: &DeviceStore,
) {
    let devices_clone = Arc::clone(devices);
    let mut introspect = context.introspect();

    let device_info = {
        if let Ok(devices_guard) = devices_clone.read() {
            devices_guard
                .values()
                .find(|d| d.key() == device_key)
                .cloned()
        } else {
            return;
        }
    };

    if let Some(info) = device_info {
        match info.key().device_type {
            DeviceType::Output => {
                introspect.set_sink_port_by_index(device_key.index, &port, None);
            }
            DeviceType::Input => {
                introspect.set_source_port_by_index(device_key.index, &port, None);
            }
        }
    }
}
