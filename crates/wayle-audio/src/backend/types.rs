use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use libpulse_binding::{
    context::subscribe::{Facility, Operation},
    volume::ChannelVolumes,
};
use tokio::sync::{broadcast, mpsc};

use super::commands::Command;
use crate::{
    events::AudioEvent,
    types::{
        device::{Device, DeviceKey},
        stream::{StreamInfo, StreamKey},
    },
};

/// Storage for audio devices
pub(crate) type DeviceStore = Arc<RwLock<HashMap<DeviceKey, Device>>>;

/// Storage for audio streams
pub(crate) type StreamStore = Arc<RwLock<HashMap<StreamKey, StreamInfo>>>;

/// Storage for default device information
pub(crate) type DefaultDevice = Arc<RwLock<Option<Device>>>;

/// Channel sender for audio events
pub(crate) type EventSender = broadcast::Sender<AudioEvent>;

/// Channel sender for backend commands
pub(crate) type CommandSender = mpsc::UnboundedSender<Command>;

/// Channel receiver for backend commands
pub(crate) type CommandReceiver = mpsc::UnboundedReceiver<Command>;

/// Channel sender for internal backend commands
pub(super) type InternalCommandSender = mpsc::UnboundedSender<InternalRefresh>;

/// Change notifications from PulseAudio subscription
#[derive(Debug, Clone)]
pub(crate) enum ChangeNotification {
    /// Device-related change notification
    Device {
        /// PulseAudio facility type
        facility: Facility,
        /// Operation performed on the device
        operation: Operation,
        /// Device index
        index: u32,
    },
    /// Stream-related change notification
    Stream {
        /// PulseAudio facility type
        facility: Facility,
        /// Operation performed on the stream
        operation: Operation,
        /// Stream index
        index: u32,
    },
    /// Server-related change notification
    Server {
        /// Operation performed on the server
        operation: Operation,
    },
}

/// Internal commands triggered by PulseAudio events
#[derive(Debug)]
pub(crate) enum InternalRefresh {
    /// Refresh device information after change notification
    Devices,
    /// Refresh stream information after change notification
    Streams,
    /// Refresh server info for default device updates
    ServerInfo,
    /// Refresh a specific device
    Device {
        /// Device key to refresh
        device_key: DeviceKey,
        /// Facility type (Sink or Source)
        facility: Facility,
    },
    /// Refresh a specific stream
    Stream {
        /// Stream key to refresh
        stream_key: StreamKey,
        /// Facility type (SinkInput or SourceOutput)
        facility: Facility,
    },
}

/// External commands from service requests
#[derive(Debug)]
pub(crate) enum ExternalCommand {
    /// Set device volume
    SetDeviceVolume {
        /// Target device
        device_key: DeviceKey,
        /// New volume levels
        volume: ChannelVolumes,
    },
    /// Set device mute state
    SetDeviceMute {
        /// Target device
        device_key: DeviceKey,
        /// Mute state
        muted: bool,
    },
    /// Set stream volume
    SetStreamVolume {
        /// Target stream
        stream_key: StreamKey,
        /// New volume levels
        volume: ChannelVolumes,
    },
    /// Set stream mute state
    SetStreamMute {
        /// Target stream
        stream_key: StreamKey,
        /// Mute state
        muted: bool,
    },
    /// Set default input device
    SetDefaultInput {
        /// Target device
        device_key: DeviceKey,
    },
    /// Set default output device
    SetDefaultOutput {
        /// Target device
        device_key: DeviceKey,
    },
    /// Move stream to different device
    MoveStream {
        /// Target stream
        stream_key: StreamKey,
        /// Destination device
        device_key: DeviceKey,
    },
    /// Set device port
    SetPort {
        /// Device to modify.
        device_key: DeviceKey,
        /// Port name to activate.
        port: String,
    },
}
