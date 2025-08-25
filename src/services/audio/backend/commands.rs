use tokio::sync::oneshot;

use crate::services::{
    AudioError,
    audio::{
        Volume,
        types::{Device, DeviceKey, StreamInfo, StreamKey},
    },
};

/// Backend command with responders for queries
#[derive(Debug)]
pub enum Command {
    /// Get device information
    GetDevice {
        /// Device to query.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<Device, AudioError>>,
    },
    /// Get stream information
    GetStream {
        /// Stream to query.
        stream_key: StreamKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<StreamInfo, AudioError>>,
    },
    /// List all devices
    ListDevices {
        /// Channel to send response.
        responder: oneshot::Sender<Result<Vec<Device>, AudioError>>,
    },
    /// List all streams
    ListStreams {
        /// Channel to send response.
        responder: oneshot::Sender<Result<Vec<StreamInfo>, AudioError>>,
    },
    /// Set device volume
    SetVolume {
        /// Device to modify.
        device_key: DeviceKey,
        /// New volume level.
        volume: Volume,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set device mute state
    SetMute {
        /// Device to modify.
        device_key: DeviceKey,
        /// New mute state.
        muted: bool,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set stream volume
    SetStreamVolume {
        /// Stream to modify.
        stream_key: StreamKey,
        /// New volume level.
        volume: Volume,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set stream mute state
    SetStreamMute {
        /// Stream to modify.
        stream_key: StreamKey,
        /// New mute state.
        muted: bool,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set default input device
    SetDefaultInput {
        /// Device to set as default.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set default output device
    SetDefaultOutput {
        /// Device to set as default.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Move stream to different device
    MoveStream {
        /// Stream to move.
        stream_key: StreamKey,
        /// Target device.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Set device port
    SetPort {
        /// Device to modify.
        device_key: DeviceKey,
        /// Port name to activate.
        port: String,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), AudioError>>,
    },
    /// Shutdown backend
    Shutdown,
}
