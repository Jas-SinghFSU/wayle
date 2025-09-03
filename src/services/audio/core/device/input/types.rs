use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::{CommandSender, EventSender},
    types::DeviceKey,
};

pub(crate) struct InputDeviceParams<'a> {
    pub command_tx: &'a CommandSender,
    pub device_key: DeviceKey,
}

pub(crate) struct LiveInputDeviceParams<'a> {
    pub command_tx: &'a CommandSender,
    pub event_tx: &'a EventSender,
    pub device_key: DeviceKey,
    pub cancellation_token: &'a CancellationToken,
}
