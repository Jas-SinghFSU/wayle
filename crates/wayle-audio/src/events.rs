use super::types::{
    device::{Device, DeviceKey},
    stream::{StreamInfo, StreamKey},
};

/// Audio system events emitted when PulseAudio state changes.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum AudioEvent {
    /// Device was added
    DeviceAdded(Device),

    /// Device properties changed
    DeviceChanged(Device),

    /// Device was removed
    DeviceRemoved(DeviceKey),

    /// Stream was added
    StreamAdded(StreamInfo),

    /// Stream properties changed
    StreamChanged(StreamInfo),

    /// Stream was removed
    StreamRemoved(StreamKey),

    /// Default input device changed
    DefaultInputChanged(Option<Device>),

    /// Default output device changed
    DefaultOutputChanged(Option<Device>),
}
