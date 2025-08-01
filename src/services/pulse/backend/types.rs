use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use libpulse_binding::{
    context::subscribe::{Facility, Operation},
    volume::ChannelVolumes,
};
use tokio::sync::{broadcast, mpsc};

use crate::services::{
    AudioEvent, DeviceInfo, StreamInfo,
    pulse::{device::DeviceKey, stream::StreamKey},
};

/// Thread-safe storage for audio devices
pub type DeviceStore = Arc<RwLock<HashMap<DeviceKey, DeviceInfo>>>;

/// Thread-safe storage for audio streams  
pub type StreamStore = Arc<RwLock<HashMap<StreamKey, StreamInfo>>>;

/// Thread-safe storage for default device information
pub type DefaultDevice = Arc<RwLock<Option<DeviceInfo>>>;

/// Channel sender for audio events
pub type EventSender = broadcast::Sender<AudioEvent>;

/// Channel sender for device list updates
pub type DeviceListSender = broadcast::Sender<Vec<DeviceInfo>>;

/// Channel sender for stream list updates
pub type StreamListSender = broadcast::Sender<Vec<StreamInfo>>;

/// Channel sender for external backend commands
pub type CommandSender = mpsc::UnboundedSender<ExternalCommand>;

/// Channel sender for internal backend commands
pub(super) type InternalCommandSender = mpsc::UnboundedSender<InternalCommand>;

/// Thread-safe storage for server information
pub type ServerInfo = Arc<RwLock<Option<String>>>;

pub(super) type ExternalCommandReceiver = mpsc::UnboundedReceiver<ExternalCommand>;
pub(super) type InternalCommandReceiver = mpsc::UnboundedReceiver<InternalCommand>;

/// Change notifications from PulseAudio subscription
#[derive(Debug, Clone)]
pub enum ChangeNotification {
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
        /// PulseAudio facility type
        facility: Facility,
        /// Operation performed on the server
        operation: Operation,
        /// Server index
        index: u32,
    },
}

/// Internal commands triggered by PulseAudio events
#[derive(Debug)]
pub enum InternalCommand {
    /// Refresh device information after change notification
    RefreshDevices,
    /// Refresh stream information after change notification
    RefreshStreams,
    /// Refresh server info for default device updates
    RefreshServerInfo,
}

/// External commands from user/API requests
#[derive(Debug)]
pub enum ExternalCommand {
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
    /// Shutdown backend
    Shutdown,
}
