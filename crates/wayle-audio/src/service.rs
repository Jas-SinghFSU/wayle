use std::sync::Arc;

use derive_more::Debug;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;
use wayle_traits::Reactive;
use zbus::Connection;

use super::core::{
    device::{
        input::{InputDeviceParams, LiveInputDeviceParams},
        output::{LiveOutputDeviceParams, OutputDeviceParams},
    },
    stream::{AudioStreamParams, LiveAudioStreamParams},
};
use crate::{
    backend::types::{CommandSender, EventSender},
    builder::AudioServiceBuilder,
    core::{
        device::{input::InputDevice, output::OutputDevice},
        stream::AudioStream,
    },
    error::Error,
    types::{device::DeviceKey, stream::StreamKey},
};

/// Audio service with reactive properties.
///
/// Provides access to audio devices and streams through reactive Property fields
/// that automatically update when the underlying PulseAudio state changes.
#[derive(Debug)]
pub struct AudioService {
    #[debug(skip)]
    pub(crate) command_tx: CommandSender,
    #[debug(skip)]
    pub(crate) event_tx: EventSender,
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) _connection: Option<Connection>,

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
    /// Creates a new audio service instance with default configuration.
    ///
    /// Initializes PulseAudio connection and discovers available devices and streams.
    ///
    /// # Errors
    /// Returns error if PulseAudio connection fails or service initialization fails.
    #[instrument]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Creates a builder for configuring an AudioService.
    pub fn builder() -> AudioServiceBuilder {
        AudioServiceBuilder::new()
    }

    /// Get a specific output device.
    ///
    /// # Errors
    /// Returns error if device not found or backend query fails.
    #[instrument(skip(self), fields(device_key = ?key), err)]
    pub async fn output_device(&self, key: DeviceKey) -> Result<OutputDevice, Error> {
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
    #[instrument(skip(self), fields(device_key = ?key), err)]
    pub async fn output_device_monitored(
        &self,
        key: DeviceKey,
    ) -> Result<Arc<OutputDevice>, Error> {
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
    #[instrument(skip(self), fields(device_key = ?key), err)]
    pub async fn input_device(&self, key: DeviceKey) -> Result<InputDevice, Error> {
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
    #[instrument(skip(self), fields(device_key = ?key), err)]
    pub async fn input_device_monitored(&self, key: DeviceKey) -> Result<Arc<InputDevice>, Error> {
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
    #[instrument(skip(self), fields(stream_key = ?key), err)]
    pub async fn audio_stream(&self, key: StreamKey) -> Result<AudioStream, Error> {
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
    #[instrument(skip(self), fields(stream_key = ?key), err)]
    pub async fn audio_stream_monitored(&self, key: StreamKey) -> Result<Arc<AudioStream>, Error> {
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
