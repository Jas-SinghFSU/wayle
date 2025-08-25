use libpulse_binding::context::{
    Context,
    subscribe::{Facility, InterestMaskSet, Operation},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::info;

use super::types::{
    ChangeNotification, DeviceStore, EventSender, InternalCommand, InternalCommandSender,
    StreamStore,
};
use crate::services::{
    AudioError, AudioEvent, DeviceType, StreamType,
    audio::types::{DeviceKey, StreamKey},
};

type SubscriptionCallback = Option<Box<dyn FnMut(Option<Facility>, Option<Operation>, u32)>>;

/// Start the event processor task
///
/// This function:
/// 1. Sets up PulseAudio event subscription
/// 2. Spawns a task to process change notifications
/// 3. Manages its own lifecycle
///
/// # Errors
/// Returns error if PulseAudio subscription setup fails
pub fn start_event_processor(
    context: &mut Context,
    devices: DeviceStore,
    streams: StreamStore,
    events_tx: EventSender,
    internal_command_tx: InternalCommandSender,
    cancellation_token: CancellationToken,
) -> Result<(), AudioError> {
    let (change_tx, mut change_rx) = mpsc::unbounded_channel::<ChangeNotification>();

    setup_subscription(context, change_tx)?;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    info!("Event processor cancelled, stopping");
                    return;
                }
                Some(notification) = change_rx.recv() => {
                    process_change_notification(
                        notification,
                        &devices,
                        &streams,
                        &events_tx,
                        &internal_command_tx
                    )
                    .await;
                }
                else => {
                    info!("Change notification channel closed");
                    return;
                }
            }
        }
    });

    Ok(())
}

/// Setup PulseAudio event subscription (internal)
fn setup_subscription(
    context: &mut Context,
    change_tx: mpsc::UnboundedSender<ChangeNotification>,
) -> Result<(), AudioError> {
    let interest_mask = InterestMaskSet::SINK
        | InterestMaskSet::SOURCE
        | InterestMaskSet::SINK_INPUT
        | InterestMaskSet::SOURCE_OUTPUT
        | InterestMaskSet::SERVER;

    let subscription_callback: SubscriptionCallback =
        Some(Box::new(move |facility, operation, index| {
            let (Some(facility), Some(operation)) = (facility, operation) else {
                return;
            };

            let notification = match facility {
                Facility::Sink | Facility::Source => ChangeNotification::Device {
                    facility,
                    operation,
                    index,
                },
                Facility::SinkInput | Facility::SourceOutput => ChangeNotification::Stream {
                    facility,
                    operation,
                    index,
                },
                Facility::Server => ChangeNotification::Server {
                    facility,
                    operation,
                    index,
                },
                _ => return,
            };

            let _ = change_tx.send(notification);
        }));

    context.set_subscribe_callback(subscription_callback);

    context.subscribe(interest_mask, |_success: bool| {});

    Ok(())
}

/// Process change notifications from PulseAudio (internal)
#[allow(clippy::too_many_arguments)]
async fn process_change_notification(
    notification: ChangeNotification,
    devices: &DeviceStore,
    streams: &StreamStore,
    events_tx: &EventSender,
    command_tx: &InternalCommandSender,
) {
    match notification {
        ChangeNotification::Device {
            facility,
            operation,
            index,
        } => {
            handle_device_change(facility, operation, index, devices, events_tx, command_tx).await;
        }
        ChangeNotification::Stream {
            facility,
            operation,
            index,
        } => {
            handle_stream_change(facility, operation, index, streams, events_tx, command_tx).await;
        }
        ChangeNotification::Server { operation, .. } => {
            handle_server_change(operation, command_tx).await;
        }
    }
}

async fn handle_device_change(
    facility: Facility,
    operation: Operation,
    index: u32,
    devices: &DeviceStore,
    events_tx: &EventSender,
    command_tx: &InternalCommandSender,
) {
    let device_type = match facility {
        Facility::Sink => DeviceType::Output,
        Facility::Source => DeviceType::Input,
        _ => return,
    };
    let device_key = DeviceKey::new(index, device_type);

    // OPTIMIZE: Each individual device sends a RefreshDevices which could be further optimized
    // by adding more granularity to the changes. This approach is currently fine for now since
    // the amount of device changes is often low. But if this becomes a problem, we can tackle it
    // later.
    match operation {
        Operation::Removed => {
            let removed_device = if let Ok(mut devices_guard) = devices.write() {
                devices_guard.remove(&device_key)
            } else {
                None
            };

            if removed_device.is_some() {
                let _ = events_tx.send(AudioEvent::DeviceRemoved(device_key));
            }
        }
        Operation::New => {
            let _ = command_tx.send(InternalCommand::RefreshDevices);
        }
        Operation::Changed => {
            let _ = command_tx.send(InternalCommand::RefreshDevices);
        }
    }
}

async fn handle_stream_change(
    facility: Facility,
    operation: Operation,
    stream_index: u32,
    streams: &StreamStore,
    events_tx: &EventSender,
    command_tx: &InternalCommandSender,
) {
    let stream_type = match facility {
        Facility::SinkInput => StreamType::Playback,
        Facility::SourceOutput => StreamType::Record,
        _ => return,
    };

    let stream_key = StreamKey {
        stream_type,
        index: stream_index,
    };

    // OPTIMIZE: Streams can be further optimized to refresh individual items instead of the whole
    // list of streams. If this becomes a bottleneck (unlikely) we can make these events more
    // granular.
    match operation {
        Operation::Removed => {
            let removed_stream = if let Ok(mut streams_guard) = streams.write() {
                streams_guard.remove(&stream_key)
            } else {
                None
            };

            if removed_stream.is_some() {
                let _ = events_tx.send(AudioEvent::StreamRemoved(stream_key));
            }
        }
        Operation::New => {
            let _ = command_tx.send(InternalCommand::RefreshStreams);
        }
        Operation::Changed => {
            let _ = command_tx.send(InternalCommand::RefreshStreams);
        }
    }
}

async fn handle_server_change(operation: Operation, command_tx: &InternalCommandSender) {
    if operation == Operation::Changed {
        let _ = command_tx.send(InternalCommand::RefreshServerInfo);
    }
}
