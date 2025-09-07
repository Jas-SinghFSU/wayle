use std::{collections::HashMap, sync::Arc};

use tracing::info;

use crate::services::{
    audio::{
        core::{
            device::{input::InputDevice, output::OutputDevice},
            stream::AudioStream,
        },
        error::AudioError,
        events::AudioEvent,
        service::AudioService,
        types::{
            device::{Device, DeviceKey},
            stream::{StreamKey, StreamType},
        },
    },
    common::property::Property,
    traits::{ModelMonitoring, ServiceMonitoring},
};

impl ServiceMonitoring for AudioService {
    type Error = AudioError;

    #[allow(clippy::too_many_lines)]
    async fn start_monitoring(&self) -> Result<(), Self::Error> {
        let mut event_rx = self.event_tx.subscribe();
        let mut output_devs: HashMap<DeviceKey, Arc<OutputDevice>> = HashMap::new();
        let mut input_devs: HashMap<DeviceKey, Arc<InputDevice>> = HashMap::new();
        let mut streams: HashMap<StreamKey, Arc<AudioStream>> = HashMap::new();

        let command_tx = self.command_tx.clone();
        let event_tx = self.event_tx.clone();
        let output_devices = self.output_devices.clone();
        let input_devices = self.input_devices.clone();
        let playback_streams = self.playback_streams.clone();
        let recording_streams = self.recording_streams.clone();
        let default_input = self.default_input.clone();
        let default_output = self.default_output.clone();
        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("AudioMonitoring cancelled, stopping");
                        return;
                    }
                    Ok(event) = event_rx.recv() => {
                        match event {
                            AudioEvent::DeviceAdded(device) => {
                                match device {
                                    Device::Sink(sink) => {
                                        let key = sink.key();
                                        let output = Arc::new(OutputDevice::from_sink(
                                            &sink,
                                            command_tx.clone(),
                                            Some(event_tx.clone()),
                                            Some(cancellation_token.child_token()),
                                        ));
                                        output.clone().start_monitoring().await.ok();
                                        output_devs.insert(key, output);
                                        output_devices.set(output_devs.values().cloned().collect());
                                    }
                                    Device::Source(source) => {
                                        let key = source.key();
                                        let input = Arc::new(InputDevice::from_source(
                                            &source,
                                            command_tx.clone(),
                                            Some(event_tx.clone()),
                                            Some(cancellation_token.child_token()),
                                        ));
                                        input.clone().start_monitoring().await.ok();
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
                                            let output = Arc::new(OutputDevice::from_sink(
                                                &sink,
                                                command_tx.clone(),
                                                Some(event_tx.clone()),
                                                Some(cancellation_token.child_token()),
                                            ));
                                            output.clone().start_monitoring().await.ok();
                                            output_devs.insert(key, output);
                                            output_devices.set(output_devs.values().cloned().collect());
                                        }
                                    }
                                    Device::Source(source) => {
                                        let key = source.key();
                                        if let Some(existing) = input_devs.get(&key) {
                                            existing.update_from_source(&source);
                                        } else {
                                            let input = Arc::new(InputDevice::from_source(
                                                &source,
                                                command_tx.clone(),
                                                Some(event_tx.clone()),
                                                Some(cancellation_token.child_token()),
                                            ));
                                            input.clone().start_monitoring().await.ok();
                                            input_devs.insert(key, input);
                                            input_devices.set(input_devs.values().cloned().collect());
                                        }
                                    }
                                }
                            }

                            AudioEvent::DeviceRemoved(key) => {
                                if let Some(device) =  output_devs.remove(&key) {
                                    if let Some(ref cancel_token) = device.cancellation_token {
                                        cancel_token.cancel();
                                    };

                                    output_devices.set(output_devs.values().cloned().collect());
                                }
                                if input_devs.remove(&key).is_some() {
                                    input_devices.set(input_devs.values().cloned().collect());
                                }
                            }

                            AudioEvent::StreamAdded(info) => {
                                let stream = Arc::new(AudioStream::from_info(
                                    info.clone(),
                                    command_tx.clone(),
                                    Some(event_tx.clone()),
                                    Some(cancellation_token.child_token()),
                                ));
                                stream.clone().start_monitoring().await.ok();
                                streams.insert(info.key(), stream);
                                update_stream_properties(&streams, &playback_streams, &recording_streams);
                            }

                            AudioEvent::StreamChanged(info) => {
                                let key = info.key();
                                if let Some(existing) = streams.get(&key) {
                                    existing.update_from_info(&info);
                                } else {
                                    let stream = Arc::new(AudioStream::from_info(
                                        info.clone(),
                                        command_tx.clone(),
                                        Some(event_tx.clone()),
                                        Some(cancellation_token.child_token()),
                                    ));
                                    stream.clone().start_monitoring().await.ok();
                                    streams.insert(key, stream);
                                    update_stream_properties(&streams, &playback_streams, &recording_streams);
                                }
                            }

                            AudioEvent::StreamRemoved(key) => {
                                if let Some(cancel_token) = streams
                                    .remove(&key)
                                    .and_then(|stream| stream.cancellation_token.clone())
                                {
                                        cancel_token.cancel();
                                }
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
                        }
                    }
                }
            }
        });

        Ok(())
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
