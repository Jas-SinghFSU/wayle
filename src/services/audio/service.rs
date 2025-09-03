use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

use super::core::{
    device::{
        input::{InputDeviceParams, LiveInputDeviceParams},
        output::{LiveOutputDeviceParams, OutputDeviceParams},
    },
    stream::{AudioStreamParams, LiveAudioStreamParams},
};
use crate::services::{
    audio::{
        backend::{
            PulseBackend,
            types::{CommandSender, EventSender},
        },
        core::{AudioStream, InputDevice, OutputDevice},
        error::AudioError,
        types::{DeviceKey, StreamKey},
    },
    common::Property,
    traits::{Reactive, ServiceMonitoring},
};

/// Audio service with reactive properties.
///
/// Provides access to audio devices and streams through reactive Property fields
/// that automatically update when the underlying PulseAudio state changes.
pub struct AudioService {
    pub(crate) command_tx: CommandSender,
    pub(crate) event_tx: EventSender,
    pub(crate) cancellation_token: CancellationToken,

    /// Output devices (speakers, headphones)
    pub output_devices: Property<Vec<Arc<OutputDevice>>>,

    /// Input devices (microphones)
    pub input_devices: Property<Vec<Arc<InputDevice>>>,

    /// Default output device
    pub default_output: Property<Option<Arc<OutputDevice>>>,

    /// Default input device
    pub default_input: Property<Option<Arc<InputDevice>>>,

    /// Playback streams
    pub playback_streams: Property<Vec<Arc<AudioStream>>>,

    /// Recording streams
    pub recording_streams: Property<Vec<Arc<AudioStream>>>,
}

impl AudioService {
    /// Creates a new audio service instance.
    ///
    /// Initializes PulseAudio connection and discovers available devices and streams.
    ///
    /// # Errors
    /// Returns error if PulseAudio connection fails or service initialization fails.
    pub async fn new() -> Result<Self, AudioError> {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (event_tx, _) = broadcast::channel(100);
        let cancellation_token = CancellationToken::new();

        let output_devices = Property::new(Vec::new());
        let input_devices = Property::new(Vec::new());
        let default_output = Property::new(None);
        let default_input = Property::new(None);
        let playback_streams = Property::new(Vec::new());
        let recording_streams = Property::new(Vec::new());

        PulseBackend::start(
            command_rx,
            event_tx.clone(),
            cancellation_token.child_token(),
        )
        .await?;

        let service = Self {
            command_tx,
            event_tx,
            cancellation_token,
            output_devices,
            input_devices,
            default_output,
            default_input,
            playback_streams,
            recording_streams,
        };

        service.start_monitoring().await?;

        Ok(service)
    }

    /// Get a specific output device.
    ///
    /// # Errors
    /// Returns error if device not found or backend query fails.
    pub async fn output_device(&self, key: DeviceKey) -> Result<OutputDevice, AudioError> {
        OutputDevice::get(OutputDeviceParams {
            command_tx: &self.command_tx,
            device_key: key,
        })
        .await
    }

    /// Get a specific output device with monitoring.
    ///
    /// # Errors
    /// Returns error if device not found, backend query fails, or monitoring setup fails.
    pub async fn output_device_monitored(
        &self,
        key: DeviceKey,
    ) -> Result<Arc<OutputDevice>, AudioError> {
        OutputDevice::get_live(LiveOutputDeviceParams {
            command_tx: &self.command_tx,
            event_tx: &self.event_tx,
            device_key: key,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Get a specific input device.
    ///
    /// # Errors
    /// Returns error if device not found or backend query fails.
    pub async fn input_device(&self, key: DeviceKey) -> Result<InputDevice, AudioError> {
        InputDevice::get(InputDeviceParams {
            command_tx: &self.command_tx,
            device_key: key,
        })
        .await
    }

    /// Get a specific input device with monitoring.
    ///
    /// # Errors
    /// Returns error if device not found, backend query fails, or monitoring setup fails.
    pub async fn input_device_monitored(
        &self,
        key: DeviceKey,
    ) -> Result<Arc<InputDevice>, AudioError> {
        InputDevice::get_live(LiveInputDeviceParams {
            command_tx: &self.command_tx,
            event_tx: &self.event_tx,
            device_key: key,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }

    /// Get a specific audio stream.
    ///
    /// # Errors
    /// Returns error if stream not found or backend query fails.
    pub async fn audio_stream(&self, key: StreamKey) -> Result<AudioStream, AudioError> {
        AudioStream::get(AudioStreamParams {
            command_tx: &self.command_tx,
            stream_key: key,
        })
        .await
    }

    /// Get a specific audio stream with monitoring.
    ///
    /// # Errors
    /// Returns error if stream not found, backend query fails, or monitoring setup fails.
    pub async fn audio_stream_monitored(
        &self,
        key: StreamKey,
    ) -> Result<Arc<AudioStream>, AudioError> {
        AudioStream::get_live(LiveAudioStreamParams {
            command_tx: &self.command_tx,
            event_tx: &self.event_tx,
            stream_key: key,
            cancellation_token: &self.cancellation_token,
        })
        .await
    }
}

impl Drop for AudioService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
