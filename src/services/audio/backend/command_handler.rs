use std::sync::Arc;

use libpulse_binding::{callbacks::ListResult, context::Context, volume::ChannelVolumes};

use super::{
    conversion::{
        create_device_info_from_sink, create_device_info_from_source,
        create_stream_info_from_sink_input, create_stream_info_from_source_output,
    },
    types::{
        DefaultDevice, DeviceStore, EventSender, ExternalCommand, InternalCommand, StreamStore,
    },
};
use crate::services::{
    AudioEvent,
    audio::types::{Device, DeviceKey, DeviceType, StreamKey, StreamType},
};

/// Handle internal PulseAudio commands (event-driven)
#[allow(clippy::too_many_arguments)]
pub fn handle_internal_command(
    context: &mut Context,
    command: InternalCommand,
    devices: &DeviceStore,
    streams: &StreamStore,
    events_tx: &EventSender,
    default_input: &DefaultDevice,
    default_output: &DefaultDevice,
) {
    match command {
        InternalCommand::RefreshDevices => {
            trigger_device_discovery(context, devices, events_tx);
        }
        InternalCommand::RefreshStreams => {
            trigger_stream_discovery(context, streams, events_tx);
        }
        InternalCommand::RefreshServerInfo => {
            trigger_server_info_query(context, events_tx, default_input, default_output);
        }
    }
}

fn trigger_device_discovery(context: &Context, devices: &DeviceStore, events_tx: &EventSender) {
    let introspect = context.introspect();

    let devices_clone = Arc::clone(devices);
    let events_tx_clone = events_tx.clone();
    introspect.get_sink_info_list(move |sink_list| {
        if let ListResult::Item(sink) = sink_list {
            let sink_info = create_device_info_from_sink(sink);
            let device_key = sink_info.key();
            let device = Device::Sink(sink_info);

            let is_new = if let Ok(mut devices_guard) = devices_clone.write() {
                let existing = devices_guard.get(&device_key).cloned();
                devices_guard.insert(device_key, device.clone());
                existing.is_none()
            } else {
                false
            };

            let event = if is_new {
                AudioEvent::DeviceAdded(device)
            } else {
                AudioEvent::DeviceChanged(device)
            };
            let _ = events_tx_clone.send(event);
        }
    });

    let devices_clone = Arc::clone(devices);
    let events_tx_clone = events_tx.clone();
    introspect.get_source_info_list(move |source_list| {
        if let ListResult::Item(source) = source_list {
            let source_info = create_device_info_from_source(source);
            let device_key = source_info.key();
            let device = Device::Source(source_info);

            let is_new = if let Ok(mut devices_guard) = devices_clone.write() {
                let existing = devices_guard.get(&device_key).cloned();
                devices_guard.insert(device_key, device.clone());
                existing.is_none()
            } else {
                false
            };

            let event = if is_new {
                AudioEvent::DeviceAdded(device)
            } else {
                AudioEvent::DeviceChanged(device)
            };
            let _ = events_tx_clone.send(event);
        }
    });
}

fn trigger_stream_discovery(context: &Context, streams: &StreamStore, events_tx: &EventSender) {
    let introspect = context.introspect();

    let streams_clone = Arc::clone(streams);
    let events_tx_clone = events_tx.clone();
    introspect.get_sink_input_info_list(move |sink_input_list| {
        if let ListResult::Item(sink_input) = sink_input_list {
            let stream_info = create_stream_info_from_sink_input(sink_input);
            let stream_key = stream_info.key();

            let is_new = if let Ok(mut streams_guard) = streams_clone.write() {
                let existing = streams_guard.get(&stream_key).cloned();
                streams_guard.insert(stream_key, stream_info.clone());
                existing.is_none()
            } else {
                false
            };

            let event = if is_new {
                AudioEvent::StreamAdded(stream_info)
            } else {
                AudioEvent::StreamChanged(stream_info)
            };
            let _ = events_tx_clone.send(event);
        }
    });

    let streams_clone = Arc::clone(streams);
    let events_tx_clone = events_tx.clone();
    introspect.get_source_output_info_list(move |source_output_list| {
        if let ListResult::Item(source_output) = source_output_list {
            let stream_info = create_stream_info_from_source_output(source_output);
            let stream_key = stream_info.key();

            let is_new = if let Ok(mut streams_guard) = streams_clone.write() {
                let existing = streams_guard.get(&stream_key).cloned();
                streams_guard.insert(stream_key, stream_info.clone());
                existing.is_none()
            } else {
                false
            };

            let event = if is_new {
                AudioEvent::StreamAdded(stream_info)
            } else {
                AudioEvent::StreamChanged(stream_info)
            };
            let _ = events_tx_clone.send(event);
        }
    });
}

fn trigger_server_info_query(
    context: &Context,
    events_tx: &EventSender,
    default_input: &DefaultDevice,
    default_output: &DefaultDevice,
) {
    let _ = context;

    let default_output_info = if let Ok(guard) = default_output.read() {
        guard.clone()
    } else {
        None
    };
    let _ = events_tx.send(AudioEvent::DefaultOutputChanged(default_output_info));

    let default_input_info = if let Ok(guard) = default_input.read() {
        guard.clone()
    } else {
        None
    };
    let _ = events_tx.send(AudioEvent::DefaultInputChanged(default_input_info));
}

/// Handle external PulseAudio commands (user-initiated)
pub fn handle_external_command(
    context: &mut Context,
    command: ExternalCommand,
    devices: &DeviceStore,
    streams: &StreamStore,
) {
    match command {
        ExternalCommand::SetDeviceVolume { device_key, volume } => {
            set_device_volume(context, device_key, volume, devices);
        }
        ExternalCommand::SetDeviceMute { device_key, muted } => {
            set_device_mute(context, device_key, muted, devices);
        }
        ExternalCommand::SetDefaultInput { device_key } => {
            set_default_input(context, device_key, devices);
        }
        ExternalCommand::SetDefaultOutput { device_key } => {
            set_default_output(context, device_key, devices);
        }
        ExternalCommand::SetStreamVolume { stream_key, volume } => {
            set_stream_volume(context, stream_key, volume, streams);
        }
        ExternalCommand::SetStreamMute { stream_key, muted } => {
            set_stream_mute(context, stream_key, muted, streams);
        }
        ExternalCommand::MoveStream {
            stream_key,
            device_key,
        } => {
            move_stream(context, stream_key, device_key, streams);
        }
        ExternalCommand::SetPort { device_key, port } => {
            set_device_port(context, device_key, port, devices);
        }
        ExternalCommand::Shutdown => {
            // Shutdown handled in main loop
        }
    }
}

fn set_device_volume(
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

fn set_device_mute(context: &Context, device_key: DeviceKey, muted: bool, devices: &DeviceStore) {
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

fn set_default_input(context: &mut Context, device_key: DeviceKey, devices: &DeviceStore) {
    if let Ok(devices_guard) = devices.read()
        && let Some(device) = devices_guard.values().find(|d| d.key() == device_key)
    {
        let name = match device {
            Device::Sink(sink) => &sink.name,
            Device::Source(source) => &source.name,
        };
        context.set_default_source(name.as_str(), |_success| {});
    }
}

fn set_default_output(context: &mut Context, device_key: DeviceKey, devices: &DeviceStore) {
    if let Ok(devices_guard) = devices.read()
        && let Some(device) = devices_guard.values().find(|d| d.key() == device_key)
    {
        let name = match device {
            Device::Sink(sink) => &sink.name,
            Device::Source(source) => &source.name,
        };
        context.set_default_sink(name.as_str(), |_success| {});
    }
}

fn set_stream_volume(
    context: &Context,
    stream_key: StreamKey,
    volume: ChannelVolumes,
    streams: &StreamStore,
) {
    let streams_clone = Arc::clone(streams);
    let mut introspect = context.introspect();

    let stream_info = {
        if let Ok(streams_guard) = streams_clone.read() {
            streams_guard.get(&stream_key).cloned()
        } else {
            return;
        }
    };

    if let Some(info) = stream_info {
        match info.stream_type() {
            StreamType::Playback => {
                introspect.set_sink_input_volume(stream_key.index, &volume, None);
            }
            StreamType::Record => {
                introspect.set_source_output_volume(stream_key.index, &volume, None);
            }
        }
    }
}

fn set_stream_mute(context: &Context, stream_key: StreamKey, muted: bool, streams: &StreamStore) {
    let streams_clone = Arc::clone(streams);
    let mut introspect = context.introspect();

    let stream_info = {
        if let Ok(streams_guard) = streams_clone.read() {
            streams_guard.get(&stream_key).cloned()
        } else {
            return;
        }
    };

    if let Some(info) = stream_info {
        match info.stream_type() {
            StreamType::Playback => {
                introspect.set_sink_input_mute(stream_key.index, muted, None);
            }
            StreamType::Record => {
                introspect.set_source_output_mute(stream_key.index, muted, None);
            }
        }
    }
}

fn move_stream(
    context: &Context,
    stream_key: StreamKey,
    device_key: DeviceKey,
    streams: &StreamStore,
) {
    let streams_clone = Arc::clone(streams);
    let mut introspect = context.introspect();

    let stream_info = {
        if let Ok(streams_guard) = streams_clone.read() {
            streams_guard.get(&stream_key).cloned()
        } else {
            return;
        }
    };

    if let Some(info) = stream_info {
        match info.stream_type() {
            StreamType::Playback => {
                introspect.move_sink_input_by_index(stream_key.index, device_key.index, None);
            }
            StreamType::Record => {
                introspect.move_source_output_by_index(stream_key.index, device_key.index, None);
            }
        }
    }
}

fn set_device_port(context: &Context, device_key: DeviceKey, port: String, devices: &DeviceStore) {
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
