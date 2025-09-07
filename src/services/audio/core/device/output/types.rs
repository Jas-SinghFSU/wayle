use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::{CommandSender, EventSender},
    types::device::DeviceKey,
};

pub(crate) struct OutputDeviceParams<'a> {
    pub command_tx: &'a CommandSender,
    pub device_key: DeviceKey,
}

pub(crate) struct LiveOutputDeviceParams<'a> {
    pub command_tx: &'a CommandSender,
    pub event_tx: &'a EventSender,
    pub device_key: DeviceKey,
    pub cancellation_token: &'a CancellationToken,
}
