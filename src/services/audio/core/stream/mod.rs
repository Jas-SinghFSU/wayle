use std::{collections::HashMap, sync::Arc};

use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

use crate::services::{
    audio::{
        Volume,
        backend::{Command, CommandSender, EventReceiver},
        error::AudioError,
        types::{ChannelMap, DeviceKey, MediaInfo, SampleSpec, StreamInfo, StreamKey, StreamState},
    },
    common::Property,
};

mod monitoring;

use monitoring::StreamMonitor;

/// Audio stream representation with reactive properties.
///
/// Provides access to stream state, volume, mute status, and media information
/// that automatically update when the underlying PulseAudio stream changes.
#[derive(Debug, Clone)]
pub struct AudioStream {
    /// Stream key for identification
    pub key: StreamKey,

    /// Stream name
    pub name: Property<String>,

    /// Application name
    pub application_name: Property<Option<String>>,

    /// Application binary path
    pub binary: Property<Option<String>>,

    /// Process ID
    pub pid: Property<Option<u32>>,

    /// Index of the owning module
    pub owner_module: Property<Option<u32>>,

    /// Index of the client this stream belongs to
    pub client: Property<Option<u32>>,

    /// Stream state
    pub state: Property<StreamState>,

    /// Current volume levels
    pub volume: Property<Volume>,

    /// Whether stream is muted
    pub muted: Property<bool>,

    /// Whether stream is corked (paused)
    pub corked: Property<bool>,

    /// Whether stream has volume control
    pub has_volume: Property<bool>,

    /// Whether volume is writable by clients
    pub volume_writable: Property<bool>,

    /// Device index this stream is connected to
    pub device_index: Property<u32>,

    /// Sample specification
    pub sample_spec: Property<SampleSpec>,

    /// Channel map
    pub channel_map: Property<ChannelMap>,

    /// Stream properties from PulseAudio
    pub properties: Property<HashMap<String, String>>,

    /// Media information
    pub media: Property<MediaInfo>,

    /// Buffer latency in microseconds
    pub buffer_latency: Property<u64>,

    /// Device latency in microseconds
    pub device_latency: Property<u64>,

    /// Resample method
    pub resample_method: Property<Option<String>>,

    /// Driver name
    pub driver: Property<String>,

    /// Format information for the stream
    pub format: Property<Option<String>>,
}

impl PartialEq for AudioStream {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl AudioStream {
    /// Get current stream state from backend (no monitoring).
    ///
    /// Queries the backend for the current stream state and returns
    /// a snapshot without setting up event monitoring.
    ///
    /// # Errors
    /// Returns error if stream not found or backend query fails.
    pub async fn get(
        command_tx: &CommandSender,
        stream_key: StreamKey,
    ) -> Result<Arc<Self>, AudioError> {
        let (tx, rx) = oneshot::channel();
        command_tx
            .send(Command::GetStream {
                stream_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        let stream_info = rx
            .await
            .map_err(|_| AudioError::BackendCommunicationFailed)??;
        Ok(Arc::new(Self::from_info(stream_info)))
    }

    /// Get stream with live monitoring.
    ///
    /// Queries the backend for current state and sets up monitoring
    /// to automatically update properties when the stream changes.
    ///
    /// # Errors
    /// Returns error if stream not found, backend query fails, or monitoring setup fails.
    pub async fn get_live(
        command_tx: &CommandSender,
        event_rx: EventReceiver,
        stream_key: StreamKey,
        cancellation_token: CancellationToken,
    ) -> Result<Arc<Self>, AudioError> {
        let stream = Self::get(command_tx, stream_key).await?;

        StreamMonitor::start(stream.clone(), stream_key, event_rx, cancellation_token).await?;

        Ok(stream)
    }

    /// Create stream from info snapshot
    pub(crate) fn from_info(info: StreamInfo) -> Self {
        Self {
            key: info.key(),
            name: Property::new(info.name),
            application_name: Property::new(info.application_name),
            binary: Property::new(info.binary),
            pid: Property::new(info.pid),
            owner_module: Property::new(info.owner_module),
            client: Property::new(info.client),
            state: Property::new(info.state),
            volume: Property::new(info.volume),
            muted: Property::new(info.muted),
            corked: Property::new(info.corked),
            has_volume: Property::new(info.has_volume),
            volume_writable: Property::new(info.volume_writable),
            device_index: Property::new(info.device_index),
            sample_spec: Property::new(info.sample_spec),
            channel_map: Property::new(info.channel_map),
            properties: Property::new(info.properties),
            media: Property::new(info.media),
            buffer_latency: Property::new(info.buffer_latency),
            device_latency: Property::new(info.device_latency),
            resample_method: Property::new(info.resample_method),
            driver: Property::new(info.driver),
            format: Property::new(info.format),
        }
    }

    /// Update stream properties from new info
    pub(crate) fn update_from_info(&self, info: &StreamInfo) {
        self.name.set(info.name.clone());
        self.application_name.set(info.application_name.clone());
        self.binary.set(info.binary.clone());
        self.pid.set(info.pid);
        self.owner_module.set(info.owner_module);
        self.client.set(info.client);
        self.state.set(info.state);
        self.volume.set(info.volume.clone());
        self.muted.set(info.muted);
        self.corked.set(info.corked);
        self.has_volume.set(info.has_volume);
        self.volume_writable.set(info.volume_writable);
        self.device_index.set(info.device_index);
        self.sample_spec.set(info.sample_spec.clone());
        self.channel_map.set(info.channel_map.clone());
        self.properties.set(info.properties.clone());
        self.media.set(info.media.clone());
        self.buffer_latency.set(info.buffer_latency);
        self.device_latency.set(info.device_latency);
        self.resample_method.set(info.resample_method.clone());
        self.driver.set(info.driver.clone());
        self.format.set(info.format.clone());
    }

    /// Set stream volume.
    ///
    /// Sends command to backend to change stream volume.
    /// Volume is a percentage from 0.0 to 1.0 (can go higher for amplification).
    ///
    /// # Errors
    /// Returns error if command send fails.
    pub async fn set_volume(
        &self,
        command_tx: &CommandSender,
        volume: f64,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();
        command_tx
            .send(Command::SetStreamVolume {
                stream_key: self.key,
                volume: Volume::from_percentage(volume, 2),
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;
        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Set stream mute state.
    ///
    /// Sends command to backend to mute or unmute stream.
    ///
    /// # Errors
    /// Returns error if command send fails.
    pub async fn set_mute(
        &self,
        command_tx: &CommandSender,
        muted: bool,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();
        command_tx
            .send(Command::SetStreamMute {
                stream_key: self.key,
                muted,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;
        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }

    /// Move stream to different device.
    ///
    /// Sends command to backend to move this stream to a different device.
    ///
    /// # Errors
    /// Returns error if command send fails or device doesn't exist.
    pub async fn move_to_device(
        &self,
        command_tx: &CommandSender,
        device_key: DeviceKey,
    ) -> Result<(), AudioError> {
        let (tx, rx) = oneshot::channel();
        command_tx
            .send(Command::MoveStream {
                stream_key: self.key,
                device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;
        rx.await
            .map_err(|_| AudioError::BackendCommunicationFailed)?
    }
}
