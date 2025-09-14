use std::collections::HashMap;

use libpulse_binding::time::MicroSeconds;

use super::format::{AudioFormat, ChannelMap, SampleSpec};
use crate::services::audio::volume::types::Volume;

/// Device state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceState {
    /// Device is running and available
    Running,
    /// Device is idle
    Idle,
    /// Device is suspended
    Suspended,
    /// Device is offline or unavailable
    Offline,
}

/// Device port information
#[derive(Debug, Clone, PartialEq)]
pub struct DevicePort {
    /// Port name
    pub name: String,
    /// Port description
    pub description: String,
    /// Port priority
    pub priority: u32,
    /// Port availability
    pub available: bool,
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Audio input device (microphone, line-in)
    Input,
    /// Audio output device (speakers, headphones)
    Output,
}

/// Device key for unique identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceKey {
    /// Device index
    pub index: u32,
    /// Device type
    pub device_type: DeviceType,
}

impl DeviceKey {
    /// Create a new device key
    pub fn new(index: u32, device_type: DeviceType) -> Self {
        Self { index, device_type }
    }
}

/// Base device information common to all devices
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DeviceInfo {
    /// Device index
    pub index: u32,
    /// Device name (internal identifier)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Card index this device belongs to
    pub card_index: Option<u32>,
    /// Index of the owning module
    pub owner_module: Option<u32>,
    /// Driver name
    pub driver: String,
    /// Device state
    pub state: DeviceState,
    /// Current volume levels
    pub volume: Volume,
    /// Base volume (reference level)
    pub base_volume: Volume,
    /// Number of volume steps for devices which do not support arbitrary volumes
    pub n_volume_steps: u32,
    /// Whether device is muted
    pub muted: bool,
    /// Device properties from PulseAudio
    pub properties: HashMap<String, String>,
    /// Available ports
    pub ports: Vec<DevicePort>,
    /// Currently active port
    pub active_port: Option<String>,
    /// Supported audio formats
    pub formats: Vec<AudioFormat>,
    /// Sample specification
    pub sample_spec: SampleSpec,
    /// Channel map
    pub channel_map: ChannelMap,
    /// Latency in microseconds
    pub latency: MicroSeconds,
    /// Configured latency in microseconds
    pub configured_latency: MicroSeconds,
    /// Device flags (raw flags from PulseAudio)
    pub flags: u32,
}

/// Sink (output) device information
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SinkInfo {
    /// Common device information
    pub device: DeviceInfo,
    /// Monitor source index
    pub monitor_source: u32,
    /// Monitor source name
    pub monitor_source_name: String,
}

/// Source (input) device information
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SourceInfo {
    /// Common device information
    pub device: DeviceInfo,
    /// Index of the sink being monitored (if this is a monitor source)
    pub monitor_of_sink: Option<u32>,
    /// Name of the sink being monitored (if this is a monitor source)
    pub monitor_of_sink_name: Option<String>,
    /// Whether this is a monitor source
    pub is_monitor: bool,
}

impl DeviceInfo {
    /// Get device key for identification (requires device type)
    pub fn key(&self, device_type: DeviceType) -> DeviceKey {
        DeviceKey {
            index: self.index,
            device_type,
        }
    }
}

impl SinkInfo {
    /// Get device key for this sink
    pub fn key(&self) -> DeviceKey {
        self.device.key(DeviceType::Output)
    }
}

impl SourceInfo {
    /// Get device key for this source
    pub fn key(&self) -> DeviceKey {
        self.device.key(DeviceType::Input)
    }
}

/// Enum wrapper for device types to handle both sinks and sources
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Device {
    /// Output device (sink)
    Sink(SinkInfo),
    /// Input device (source)
    Source(SourceInfo),
}

impl Device {
    /// Get device key for identification
    pub fn key(&self) -> DeviceKey {
        match self {
            Device::Sink(sink) => sink.key(),
            Device::Source(source) => source.key(),
        }
    }
}
