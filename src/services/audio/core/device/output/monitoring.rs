use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::EventReceiver,
    core::device::OutputDevice,
    error::AudioError,
    events::AudioEvent,
    types::{Device, DeviceKey, DeviceState},
};

/// Monitors output device events and updates properties.
pub struct OutputDeviceMonitor;

impl OutputDeviceMonitor {
    /// Start monitoring for output device changes.
    pub(super) async fn start(
        device: Arc<OutputDevice>,
        device_key: DeviceKey,
        mut event_rx: EventReceiver,
        cancellation_token: CancellationToken,
    ) -> Result<JoinHandle<()>, AudioError> {
        let weak_device = Arc::downgrade(&device);

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        tracing::debug!("OutputDevice monitor cancelled for {:?}", device_key);
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        let Some(device) = weak_device.upgrade() else {
                            break;
                        };

                        match event {
                            AudioEvent::DeviceChanged(Device::Sink(sink)) if sink.key() == device_key => {
                                device.update_from_sink(&sink);
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
