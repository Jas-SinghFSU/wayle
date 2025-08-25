use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

use crate::services::{
    audio::{
        backend::{CommandSender, EventSender, PulseBackend},
        core::{AudioStream, InputDevice, OutputDevice},
        discovery::AudioDiscovery,
        error::AudioError,
        types::{DeviceKey, StreamKey},
    },
    common::Property,
};

/// Audio service with reactive properties.
///
/// Provides access to audio devices and streams through reactive Property fields
/// that automatically update when the underlying PulseAudio state changes.
pub struct AudioService {
    command_tx: CommandSender,
    event_tx: EventSender,
    cancellation_token: CancellationToken,

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

        AudioDiscovery::start(
            event_tx.subscribe(),
            output_devices.clone(),
            input_devices.clone(),
            playback_streams.clone(),
            recording_streams.clone(),
            default_input.clone(),
            default_output.clone(),
            cancellation_token.child_token(),
        )
        .await?;

        Ok(Self {
            command_tx,
            event_tx,
            cancellation_token,
            output_devices,
            input_devices,
            default_output,
            default_input,
            playback_streams,
            recording_streams,
        })
    }

    /// Get a specific output device.
    ///
    /// # Errors
    /// Returns error if device not found or backend query fails.
    pub async fn output_device(&self, key: DeviceKey) -> Result<Arc<OutputDevice>, AudioError> {
        OutputDevice::get(&self.command_tx, key).await
    }

    /// Get a specific output device with monitoring.
    ///
    /// # Errors
    /// Returns error if device not found, backend query fails, or monitoring setup fails.
    pub async fn output_device_monitored(
        &self,
        key: DeviceKey,
    ) -> Result<Arc<OutputDevice>, AudioError> {
        OutputDevice::get_live(
            &self.command_tx,
            self.event_tx.subscribe(),
            key,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Get a specific input device.
    ///
    /// # Errors
    /// Returns error if device not found or backend query fails.
    pub async fn input_device(&self, key: DeviceKey) -> Result<Arc<InputDevice>, AudioError> {
        InputDevice::get(&self.command_tx, key).await
    }

    /// Get a specific input device with monitoring.
    ///
    /// # Errors
    /// Returns error if device not found, backend query fails, or monitoring setup fails.
    pub async fn input_device_monitored(
        &self,
        key: DeviceKey,
    ) -> Result<Arc<InputDevice>, AudioError> {
        InputDevice::get_live(
            &self.command_tx,
            self.event_tx.subscribe(),
            key,
            self.cancellation_token.child_token(),
        )
        .await
    }

    /// Get a specific audio stream.
    ///
    /// # Errors
    /// Returns error if stream not found or backend query fails.
    pub async fn audio_stream(&self, key: StreamKey) -> Result<Arc<AudioStream>, AudioError> {
        AudioStream::get(&self.command_tx, key).await
    }

    /// Get a specific audio stream with monitoring.
    ///
    /// # Errors
    /// Returns error if stream not found, backend query fails, or monitoring setup fails.
    pub async fn audio_stream_monitored(
        &self,
        key: StreamKey,
    ) -> Result<Arc<AudioStream>, AudioError> {
        AudioStream::get_live(
            &self.command_tx,
            self.event_tx.subscribe(),
            key,
            self.cancellation_token.child_token(),
        )
        .await
    }
}

impl Drop for AudioService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
