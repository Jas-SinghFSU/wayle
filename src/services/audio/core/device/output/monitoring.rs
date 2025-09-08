use std::sync::Arc;

use tracing::debug;

use crate::services::{
    audio::{
        core::device::output::OutputDevice,
        error::Error,
        events::AudioEvent,
        types::device::{Device, DeviceState},
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for OutputDevice {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::MonitoringNotInitialized(String::from(
                "Cancellation token not available",
            )));
        };

        let Some(ref event_tx) = self.event_tx else {
            return Err(Error::MonitoringNotInitialized(String::from(
                "Event sender not available",
            )));
        };

        let weak_device = Arc::downgrade(&self);
        let device_key = self.key;
        let cancellation_token = cancellation_token.clone();
        let mut event_rx = event_tx.subscribe();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        debug!("OutputDevice monitor cancelled for {:?}", device_key);
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        let Some(device) = weak_device.upgrade() else {
                            return;
                        };

                        match event {
                            AudioEvent::DeviceChanged(Device::Sink(sink))
                                if sink.key() == device_key =>
                            {
                                device.update_from_sink(&sink);
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

        Ok(())
    }
}
