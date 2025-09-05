use std::sync::Arc;

use tracing::debug;

use crate::services::{
    audio::{core::stream::AudioStream, error::AudioError, events::AudioEvent, types::StreamState},
    traits::ModelMonitoring,
};

impl ModelMonitoring for AudioStream {
    type Error = AudioError;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(AudioError::OperationFailed(String::from(
                "Cancellation token not available for monitoring",
            )));
        };

        let Some(ref event_tx) = self.event_tx else {
            return Err(AudioError::OperationFailed(String::from(
                "Event sender not available for monitoring",
            )));
        };

        let weak_stream = Arc::downgrade(&self);
        let stream_key = self.key;
        let cancellation_token = cancellation_token.clone();
        let mut event_rx = event_tx.subscribe();

        tokio::spawn(async move {
            loop {
                let Some(stream) = weak_stream.upgrade() else {
                    return;
                };

                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        debug!("AudioStream monitor cancelled for {:?}", stream_key);
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        match event {
                            AudioEvent::StreamChanged(info) if info.key() == stream_key => {
                                stream.update_from_info(&info);
                            }
                            AudioEvent::StreamVolumeChanged {
                                stream_key: key,
                                volume,
                            } if key == stream_key => {
                                stream.volume.set(volume);
                            }
                            AudioEvent::StreamMuteChanged {
                                stream_key: key,
                                muted,
                            } if key == stream_key => {
                                stream.muted.set(muted);
                            }
                            AudioEvent::StreamStateChanged {
                                stream_key: key,
                                state,
                            } if key == stream_key => {
                                stream.state.set(state);
                            }
                            AudioEvent::StreamCorkedChanged {
                                stream_key: key,
                                corked,
                            } if key == stream_key => {
                                stream.corked.set(corked);
                            }
                            AudioEvent::StreamMovedToDevice {
                                stream_key: key,
                                device_index,
                            } if key == stream_key => {
                                stream.device_index.set(device_index);
                            }
                            AudioEvent::StreamRemoved(key) if key == stream_key => {
                                stream.state.set(StreamState::Terminated);
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
