use libpulse_binding::context::subscribe::{Facility, Operation};

use crate::services::{
    AudioEvent, DeviceType,
    audio::{
        backend::types::{DeviceStore, EventSender, InternalCommand, InternalCommandSender},
        types::DeviceKey,
    },
};

pub(crate) async fn handle_device_change(
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
        Operation::New | Operation::Changed => {
            let _ = command_tx.send(InternalCommand::RefreshDevice {
                device_key,
                facility,
            });
        }
    }
}
