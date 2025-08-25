use super::{
    Volume,
    types::{Device, DeviceKey, DeviceState, StreamInfo, StreamKey, StreamState},
};

/// Audio system events emitted when PulseAudio state changes.
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

    /// Device volume changed
    DeviceVolumeChanged {
        /// Device that changed.
        device_key: DeviceKey,
        /// New volume level.
        volume: Volume,
    },

    /// Device mute state changed
    DeviceMuteChanged {
        /// Device that changed.
        device_key: DeviceKey,
        /// New mute state.
        muted: bool,
    },

    /// Device state changed
    DeviceStateChanged {
        /// Device that changed.
        device_key: DeviceKey,
        /// New device state.
        state: DeviceState,
    },

    /// Device active port changed
    DevicePortChanged {
        /// Device that changed.
        device_key: DeviceKey,
        /// New active port name.
        port_name: Option<String>,
    },

    /// Stream volume changed
    StreamVolumeChanged {
        /// Stream that changed.
        stream_key: StreamKey,
        /// New volume level.
        volume: Volume,
    },

    /// Stream mute state changed
    StreamMuteChanged {
        /// Stream that changed.
        stream_key: StreamKey,
        /// New mute state.
        muted: bool,
    },

    /// Stream state changed
    StreamStateChanged {
        /// Stream that changed.
        stream_key: StreamKey,
        /// New stream state.
        state: StreamState,
    },

    /// Stream corked state changed
    StreamCorkedChanged {
        /// Stream that changed.
        stream_key: StreamKey,
        /// New cork state.
        corked: bool,
    },

    /// Stream moved to different device
    StreamMovedToDevice {
        /// Stream that was moved.
        stream_key: StreamKey,
        /// Target device index.
        device_index: u32,
    },

    /// Default input device changed
    DefaultInputChanged(Option<Device>),

    /// Default output device changed
    DefaultOutputChanged(Option<Device>),
}
