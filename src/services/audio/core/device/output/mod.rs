pub(crate) mod controls;
pub(crate) mod monitoring;
pub(crate) mod types;

use std::{collections::HashMap, sync::Arc};

use controls::OutputDeviceController;
use libpulse_binding::time::MicroSeconds;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;
pub(crate) use types::{LiveOutputDeviceParams, OutputDeviceParams};

use crate::services::{
    audio::{
        backend::{
            commands::Command,
            types::{CommandSender, EventSender},
        },
        error::Error,
        types::{
            device::{Device, DeviceKey, DevicePort, DeviceState, DeviceType, SinkInfo},
            format::{AudioFormat, ChannelMap, SampleSpec},
        },
        volume::types::Volume,
    },
    common::property::Property,
    traits::{ModelMonitoring, Reactive},
};

/// Output device (sink) representation with reactive properties.
#[derive(Clone)]
pub struct OutputDevice {
    /// Command sender for backend operations
    command_tx: CommandSender,

    /// Event sender for monitoring (only for live instances)
    event_tx: Option<EventSender>,

    /// Cancellation token for monitoring (only for live instances)
    pub(crate) cancellation_token: Option<CancellationToken>,

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

    /// Monitor source index for this output
    pub monitor_source: Property<u32>,

    /// Monitor source name for this output
    pub monitor_source_name: Property<String>,

    /// Latency in microseconds
    pub latency: Property<MicroSeconds>,

    /// Configured latency in microseconds
    pub configured_latency: Property<MicroSeconds>,

    /// Device flags (raw flags from PulseAudio)
    pub flags: Property<u32>,
}

impl PartialEq for OutputDevice {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Reactive for OutputDevice {
    type Context<'a> = OutputDeviceParams<'a>;
    type LiveContext<'a> = LiveOutputDeviceParams<'a>;
    type Error = Error;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        let (tx, rx) = oneshot::channel();
        params
            .command_tx
            .send(Command::GetDevice {
                device_key: params.device_key,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        let device = rx
            .await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))??;

        match device {
            Device::Sink(sink) => Ok(Self::from_sink(
                &sink,
                params.command_tx.clone(),
                None,
                None,
            )),
            Device::Source(_) => Err(Error::DeviceNotFound {
                index: params.device_key.index,
                device_type: DeviceType::Output,
            }),
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
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        let device = rx
            .await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))??;

        let device = match device {
            Device::Sink(sink) => Arc::new(Self::from_sink(
                &sink,
                params.command_tx.clone(),
                Some(params.event_tx.clone()),
                Some(params.cancellation_token.child_token()),
            )),
            Device::Source(_) => {
                return Err(Error::DeviceNotFound {
                    index: params.device_key.index,
                    device_type: DeviceType::Output,
                });
            }
        };

        device.clone().start_monitoring().await?;

        Ok(device)
    }
}

impl OutputDevice {
    pub(crate) fn from_sink(
        sink: &SinkInfo,
        command_tx: CommandSender,
        event_tx: Option<EventSender>,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            command_tx,
            event_tx,
            cancellation_token,
            key: sink.key(),
            name: Property::new(sink.name.clone()),
            description: Property::new(sink.description.clone()),
            card_index: Property::new(sink.card_index),
            owner_module: Property::new(sink.owner_module),
            driver: Property::new(sink.driver.clone()),
            state: Property::new(sink.state),
            volume: Property::new(sink.volume.clone()),
            base_volume: Property::new(sink.base_volume.clone()),
            n_volume_steps: Property::new(sink.n_volume_steps),
            muted: Property::new(sink.muted),
            properties: Property::new(sink.properties.clone()),
            ports: Property::new(sink.ports.clone()),
            active_port: Property::new(sink.active_port.clone()),
            formats: Property::new(sink.formats.clone()),
            sample_spec: Property::new(sink.sample_spec.clone()),
            channel_map: Property::new(sink.channel_map.clone()),
            monitor_source: Property::new(sink.monitor_source),
            monitor_source_name: Property::new(sink.monitor_source_name.clone()),
            latency: Property::new(sink.latency),
            configured_latency: Property::new(sink.configured_latency),
            flags: Property::new(sink.flags),
        }
    }

    pub(crate) fn update_from_sink(&self, sink: &SinkInfo) {
        self.name.set(sink.name.clone());
        self.description.set(sink.description.clone());
        self.card_index.set(sink.card_index);
        self.owner_module.set(sink.owner_module);
        self.driver.set(sink.driver.clone());
        self.state.set(sink.state);
        self.volume.set(sink.volume.clone());
        self.base_volume.set(sink.base_volume.clone());
        self.n_volume_steps.set(sink.n_volume_steps);
        self.muted.set(sink.muted);
        self.properties.set(sink.properties.clone());
        self.ports.set(sink.ports.clone());
        self.active_port.set(sink.active_port.clone());
        self.formats.set(sink.formats.clone());
        self.sample_spec.set(sink.sample_spec.clone());
        self.channel_map.set(sink.channel_map.clone());
        self.monitor_source.set(sink.monitor_source);
        self.monitor_source_name
            .set(sink.monitor_source_name.clone());
        self.latency.set(sink.latency);
        self.configured_latency.set(sink.configured_latency);
        self.flags.set(sink.flags);
    }

    /// Set the volume for this output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_volume(&self, volume: Volume) -> Result<(), Error> {
        OutputDeviceController::set_volume(&self.command_tx, self.key, volume).await
    }

    /// Set the mute state for this output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_mute(&self, muted: bool) -> Result<(), Error> {
        OutputDeviceController::set_mute(&self.command_tx, self.key, muted).await
    }

    /// Set the active port for this output device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_port(&self, port: String) -> Result<(), Error> {
        OutputDeviceController::set_port(&self.command_tx, self.key, port).await
    }

    /// Set this device as the default output.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    pub async fn set_as_default(&self) -> Result<(), Error> {
        OutputDeviceController::set_as_default(&self.command_tx, self.key).await
    }
}
