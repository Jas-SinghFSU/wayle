use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::EventReceiver,
    core::device::InputDevice,
    error::AudioError,
    events::AudioEvent,
    types::{Device, DeviceKey, DeviceState},
};

/// Monitors input device events and updates properties.
pub struct InputDeviceMonitor;

impl InputDeviceMonitor {
    /// Start monitoring for input device changes.
    pub(super) async fn start(
        device: Arc<InputDevice>,
        device_key: DeviceKey,
        mut event_rx: EventReceiver,
        cancellation_token: CancellationToken,
    ) -> Result<JoinHandle<()>, AudioError> {
        let weak_device = Arc::downgrade(&device);

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        tracing::debug!("InputDevice monitor cancelled for {:?}", device_key);
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        let Some(device) = weak_device.upgrade() else {
                            break;
                        };

                        match event {
                            AudioEvent::DeviceChanged(Device::Source(source)) if source.key() == device_key => {
                                device.update_from_source(&source);
                            }
                            AudioEvent::DeviceVolumeChanged {
                                device_key: key,
                                volume,
                            } if key == device_key => {
                                device.volume.set(volume);
                            }
                            AudioEvent::DeviceMuteChanged {
                                device_key: key,
                                muted,
                            } if key == device_key => {
                                device.muted.set(muted);
                            }
                            AudioEvent::DeviceStateChanged {
                                device_key: key,
                                state,
                            } if key == device_key => {
                                device.state.set(state);
                            }
                            AudioEvent::DevicePortChanged {
                                device_key: key,
                                port_name,
                            } if key == device_key => {
                                device.active_port.set(port_name);
                            }
                            AudioEvent::DeviceRemoved(key) if key == device_key => {
                                device.state.set(DeviceState::Offline);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(handle)
    }
}
