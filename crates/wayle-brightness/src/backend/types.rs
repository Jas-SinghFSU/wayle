use tokio::sync::{broadcast, mpsc, oneshot};

use crate::{Error, types::BacklightInfo};

pub(crate) type CommandSender = mpsc::UnboundedSender<Command>;
pub(crate) type CommandReceiver = mpsc::UnboundedReceiver<Command>;
pub(crate) type EventSender = broadcast::Sender<BrightnessEvent>;

/// Commands sent from the service to the backend.
#[derive(Debug)]
pub(crate) enum Command {
    SetBrightness {
        name: String,
        value: u32,
        responder: oneshot::Sender<Result<(), Error>>,
    },
}

/// Events emitted by the backend to the monitoring loop.
#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum BrightnessEvent {
    DeviceAdded(BacklightInfo),
    DeviceChanged(BacklightInfo),
    DeviceRemoved(String),
}
