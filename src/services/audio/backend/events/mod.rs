use libpulse_binding::context::{
    Context,
    subscribe::{Facility, InterestMaskSet, Operation},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::info;

use super::types::{
    ChangeNotification, DeviceStore, EventSender, InternalCommandSender, StreamStore,
};
use crate::services::AudioError;

mod device;
mod server;
mod stream;

use device::handle_device_change;
use server::handle_server_change;
use stream::handle_stream_change;

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
