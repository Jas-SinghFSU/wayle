use std::{collections::HashMap, sync::Arc};

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::services::{
    audio::{
        backend::types::{CommandSender, EventReceiver},
        core::{AudioStream, InputDevice, OutputDevice},
        events::AudioEvent,
        types::{Device, DeviceKey, StreamKey, StreamType},
    },
    common::Property,
};

/// Audio discovery service that maintains reactive collections.
///
/// Listens to backend events and updates Property collections
/// for devices and streams.
pub struct AudioDiscovery;

impl AudioDiscovery {
    /// Start discovery service.
    ///
    /// Monitors backend events and maintains collections of devices and streams.
    ///
    /// # Errors
    /// Returns error if task spawn fails.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_lines)]
    pub async fn start(
        command_tx: CommandSender,
        mut event_rx: EventReceiver,
        output_devices: Property<Vec<Arc<OutputDevice>>>,
        input_devices: Property<Vec<Arc<InputDevice>>>,
        playback_streams: Property<Vec<Arc<AudioStream>>>,
        recording_streams: Property<Vec<Arc<AudioStream>>>,
        default_input: Property<Option<Arc<InputDevice>>>,
        default_output: Property<Option<Arc<OutputDevice>>>,
        cancellation_token: CancellationToken,
    ) -> Result<JoinHandle<()>, crate::services::audio::AudioError> {
        let handle = tokio::spawn(async move {
            let mut output_devs: HashMap<DeviceKey, Arc<OutputDevice>> = HashMap::new();
            let mut input_devs: HashMap<DeviceKey, Arc<InputDevice>> = HashMap::new();
            let mut streams: HashMap<StreamKey, Arc<AudioStream>> = HashMap::new();

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("AudioDiscovery cancelled, stopping");
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        match event {
                            AudioEvent::DeviceAdded(device) => {
                                match device {
                                    Device::Sink(sink) => {
                                        let key = sink.key();
                                        let output = Arc::new(OutputDevice::from_sink(&sink, command_tx.clone()));
                                        output_devs.insert(key, output);
                                        output_devices.set(output_devs.values().cloned().collect());
                                    }
                                    Device::Source(source) => {
                                        let key = source.key();
                                        let input = Arc::new(InputDevice::from_source(&source, command_tx.clone()));
                                        input_devs.insert(key, input);
                                        input_devices.set(input_devs.values().cloned().collect());
                                    }
                                }
                            }

                            AudioEvent::DeviceChanged(device) => {
                                match device {
                                    Device::Sink(sink) => {
                                        let key = sink.key();
                                        if let Some(existing) = output_devs.get(&key) {
                                            existing.update_from_sink(&sink);
                                        } else {
                                            let output = Arc::new(OutputDevice::from_sink(&sink, command_tx.clone()));
                                            output_devs.insert(key, output);
                                            output_devices.set(output_devs.values().cloned().collect());
                                        }
                                    }
                                    Device::Source(source) => {
                                        let key = source.key();
                                        if let Some(existing) = input_devs.get(&key) {
                                            existing.update_from_source(&source);
                                        } else {
                                            let input = Arc::new(InputDevice::from_source(&source, command_tx.clone()));
                                            input_devs.insert(key, input);
                                            input_devices.set(input_devs.values().cloned().collect());
                                        }
                                    }
                                }
                            }

                            AudioEvent::DeviceRemoved(key) => {
                                if output_devs.remove(&key).is_some() {
                                    output_devices.set(output_devs.values().cloned().collect());
                                }
                                if input_devs.remove(&key).is_some() {
                                    input_devices.set(input_devs.values().cloned().collect());
                                }
                            }

                            AudioEvent::StreamAdded(info) => {
                                let stream = Arc::new(AudioStream::from_info(info.clone(), command_tx.clone()));
                                streams.insert(info.key(), stream);
                                update_stream_properties(&streams, &playback_streams, &recording_streams);
                            }

                            AudioEvent::StreamChanged(info) => {
                                let key = info.key();
                                if let Some(existing) = streams.get(&key) {
                                    existing.update_from_info(&info);
                                } else {
                                    let stream = Arc::new(AudioStream::from_info(info.clone(), command_tx.clone()));
                                    streams.insert(key, stream);
                                    update_stream_properties(&streams, &playback_streams, &recording_streams);
                                }
                            }

                            AudioEvent::StreamRemoved(key) => {
                                streams.remove(&key);
                                update_stream_properties(&streams, &playback_streams, &recording_streams);
                            }

                            AudioEvent::DefaultInputChanged(maybe_device) => {
                                let device = maybe_device.and_then(|dev| {
                                    match dev {
                                        Device::Source(source) => {
                                            let key = source.key();
                                            input_devs.get(&key).cloned()
                                        }
                                        _ => None,
                                    }
                                });
                                default_input.set(device);
                            }

                            AudioEvent::DefaultOutputChanged(maybe_device) => {
                                let device = maybe_device.and_then(|dev| {
                                    match dev {
                                        Device::Sink(sink) => {
                                            let key = sink.key();
                                            output_devs.get(&key).cloned()
                                        }
                                        _ => None,
                                    }
                                });
                                default_output.set(device);
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

fn update_stream_properties(
    streams: &HashMap<StreamKey, Arc<AudioStream>>,
    playback_streams: &Property<Vec<Arc<AudioStream>>>,
    recording_streams: &Property<Vec<Arc<AudioStream>>>,
) {
    let playback: Vec<Arc<AudioStream>> = streams
        .values()
        .filter(|s| s.key.stream_type == StreamType::Playback)
        .cloned()
        .collect();

    let recording: Vec<Arc<AudioStream>> = streams
        .values()
        .filter(|s| s.key.stream_type == StreamType::Record)
        .cloned()
        .collect();

    playback_streams.set(playback);
    recording_streams.set(recording);
}
