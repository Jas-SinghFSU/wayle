mod controls;
mod monitoring;
mod types;

use std::{collections::HashMap, sync::Arc};

use controls::InputDeviceController;
use libpulse_binding::time::MicroSeconds;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
pub(crate) use types::{InputDeviceParams, LiveInputDeviceParams};

use crate::services::{
    audio::{
        Volume,
        backend::{
            commands::Command,
            types::{CommandSender, EventSender},
        },
        error::AudioError,
        types::{
            AudioFormat, ChannelMap, Device, DeviceKey, DevicePort, DeviceState, DeviceType,
            SampleSpec, SourceInfo,
        },
    },
    common::Property,
    traits::{ModelMonitoring, Reactive},
};

/// Input device (source) representation with reactive properties.
#[derive(Clone)]
pub struct InputDevice {
    /// Command sender for backend operations
    command_tx: CommandSender,

    /// Event sender for monitoring (only for live instances)
    event_tx: Option<EventSender>,

    /// Cancellation token for monitoring (only for live instances)
    cancellation_token: Option<CancellationToken>,

    /// Device key for identification
    pub key: DeviceKey,

    /// Device name (internal identifier)
    pub name: Property<String>,

    /// Human-readable description
    pub description: Property<String>,

    /// Card index this device belongs to
    pub card_index: Property<Option<u32>>,

    /// Index of the owning module
    pub owner_module: Property<Option<u32>>,

    /// Driver name
    pub driver: Property<String>,

    /// Device state
    pub state: Property<DeviceState>,

    /// Current volume levels
    pub volume: Property<Volume>,

    /// Base volume (reference level)
    pub base_volume: Property<Volume>,

    /// Number of volume steps for devices which do not support arbitrary volumes
    pub n_volume_steps: Property<u32>,

    /// Whether device is muted
    pub muted: Property<bool>,

    /// Device properties from PulseAudio
    pub properties: Property<HashMap<String, String>>,

    /// Available ports
    pub ports: Property<Vec<DevicePort>>,

    /// Currently active port
    pub active_port: Property<Option<String>>,

    /// Supported audio formats
    pub formats: Property<Vec<AudioFormat>>,

    /// Sample specification
    pub sample_spec: Property<SampleSpec>,

    /// Channel map
    pub channel_map: Property<ChannelMap>,

    /// Index of the sink being monitored (if this is a monitor source)
    pub monitor_of_sink: Property<Option<u32>>,

    /// Name of the sink being monitored (if this is a monitor source)
    pub monitor_of_sink_name: Property<Option<String>>,

    /// Whether this is a monitor source
    pub is_monitor: Property<bool>,

    /// Latency in microseconds
    pub latency: Property<MicroSeconds>,

    /// Configured latency in microseconds
    pub configured_latency: Property<MicroSeconds>,

    /// Device flags (raw flags from PulseAudio)
    pub flags: Property<u32>,
}

impl PartialEq for InputDevice {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Reactive for InputDevice {
    type Context<'a> = InputDeviceParams<'a>;
    type LiveContext<'a> = LiveInputDeviceParams<'a>;
    type Error = AudioError;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let (tx, rx) = oneshot::channel();
        params
            .command_tx
            .send(Command::GetDevice {
                device_key: params.device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        let device = rx
            .await
            .map_err(|_| AudioError::BackendCommunicationFailed)??;

        match device {
            Device::Source(source) => Ok(Self::from_source(
                &source,
                params.command_tx.clone(),
                None,
                None,
            )),
            Device::Sink(_) => Err(AudioError::DeviceNotFound(
                params.device_key.index,
                DeviceType::Input,
            )),
        }
    }

    async fn get_live(params: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let (tx, rx) = oneshot::channel();
        params
            .command_tx
            .send(Command::GetDevice {
                device_key: params.device_key,
                responder: tx,
            })
            .map_err(|_| AudioError::BackendCommunicationFailed)?;

        let device = rx
            .await
            .map_err(|_| AudioError::BackendCommunicationFailed)??;

        let device = match device {
            Device::Source(source) => Arc::new(Self::from_source(
                &source,
                params.command_tx.clone(),
                Some(params.event_tx.clone()),
                Some(params.cancellation_token.child_token()),
            )),
            Device::Sink(_) => {
                return Err(AudioError::DeviceNotFound(
                    params.device_key.index,
                    DeviceType::Input,
                ));
            }
        };

        device.clone().start_monitoring().await?;

        Ok(device)
    }
}

impl InputDevice {
    pub(crate) fn from_source(
        source: &SourceInfo,
        command_tx: CommandSender,
        event_tx: Option<EventSender>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            command_tx,
            event_tx,
            cancellation_token,
            key: source.key(),
            name: Property::new(source.name.clone()),
            description: Property::new(source.description.clone()),
            card_index: Property::new(source.card_index),
            owner_module: Property::new(source.owner_module),
            driver: Property::new(source.driver.clone()),
            state: Property::new(source.state),
            volume: Property::new(source.volume.clone()),
            base_volume: Property::new(source.base_volume.clone()),
            n_volume_steps: Property::new(source.n_volume_steps),
            muted: Property::new(source.muted),
            properties: Property::new(source.properties.clone()),
            ports: Property::new(source.ports.clone()),
            active_port: Property::new(source.active_port.clone()),
            formats: Property::new(source.formats.clone()),
            sample_spec: Property::new(source.sample_spec.clone()),
            channel_map: Property::new(source.channel_map.clone()),
            monitor_of_sink: Property::new(source.monitor_of_sink),
            monitor_of_sink_name: Property::new(source.monitor_of_sink_name.clone()),
            is_monitor: Property::new(source.is_monitor),
            latency: Property::new(source.latency),
            configured_latency: Property::new(source.configured_latency),
            flags: Property::new(source.flags),
        }
    }

    pub(crate) fn update_from_source(&self, source: &SourceInfo) {
        self.name.set(source.name.clone());
        self.description.set(source.description.clone());
        self.card_index.set(source.card_index);
        self.owner_module.set(source.owner_module);
        self.driver.set(source.driver.clone());
        self.state.set(source.state);
        self.volume.set(source.volume.clone());
        self.base_volume.set(source.base_volume.clone());
        self.n_volume_steps.set(source.n_volume_steps);
        self.muted.set(source.muted);
        self.properties.set(source.properties.clone());
        self.ports.set(source.ports.clone());
        self.active_port.set(source.active_port.clone());
        self.formats.set(source.formats.clone());
        self.sample_spec.set(source.sample_spec.clone());
        self.channel_map.set(source.channel_map.clone());
        self.monitor_of_sink.set(source.monitor_of_sink);
        self.monitor_of_sink_name
            .set(source.monitor_of_sink_name.clone());
        self.is_monitor.set(source.is_monitor);
        self.latency.set(source.latency);
        self.configured_latency.set(source.configured_latency);
        self.flags.set(source.flags);
    }

    /// Set the volume for this input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_volume(&self, volume: Volume) -> Result<(), AudioError> {
        InputDeviceController::set_volume(&self.command_tx, self.key, volume).await
    }

    /// Set the mute state for this input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        InputDeviceController::set_mute(&self.command_tx, self.key, muted).await
    }

    /// Set the active port for this input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_port(&self, port: String) -> Result<(), AudioError> {
        InputDeviceController::set_port(&self.command_tx, self.key, port).await
    }

    /// Set this device as the default input.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_as_default(&self) -> Result<(), AudioError> {
        InputDeviceController::set_as_default(&self.command_tx, self.key).await
    }
}
