use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::services::audio::{
    backend::types::EventReceiver, core::stream::AudioStream, error::AudioError,
    events::AudioEvent, types::StreamKey,
};

/// Monitors stream events and updates properties.
pub struct StreamMonitor;

impl StreamMonitor {
    /// Start monitoring for stream changes.
    ///
    /// Spawns a background task that listens for events related to this stream
    /// and updates the stream's properties when changes occur.
    ///
    /// # Errors
    /// Returns error if monitoring task fails to spawn.
    pub async fn start(
        stream: Arc<AudioStream>,
        stream_key: StreamKey,
        mut event_rx: EventReceiver,
        cancellation_token: CancellationToken,
    ) -> Result<JoinHandle<()>, AudioError> {
        let weak_stream = Arc::downgrade(&stream);

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        tracing::debug!("Stream monitor cancelled for {:?}", stream_key);
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        let Some(stream) = weak_stream.upgrade() else {
                            break;
                        };

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
                                stream
                                    .state
                                    .set(crate::services::audio::types::StreamState::Terminated);
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
