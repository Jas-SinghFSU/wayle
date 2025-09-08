pub(crate) mod device;
pub(crate) mod server;
pub(crate) mod stream;

use tokio::sync::oneshot;

use crate::services::audio::{
    error::Error,
    types::{
        device::{Device, DeviceKey},
        stream::{StreamInfo, StreamKey},
    },
    volume::types::Volume,
};

/// Backend command with responders for queries
#[derive(Debug)]
pub(crate) enum Command {
    /// Get device information
    GetDevice {
        /// Device to query.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<Device, Error>>,
    },
    /// Get stream information
    GetStream {
        /// Stream to query.
        stream_key: StreamKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<StreamInfo, Error>>,
    },
    /// Set device volume
    SetVolume {
        /// Device to modify.
        device_key: DeviceKey,
        /// New volume level.
        volume: Volume,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set device mute state
    SetMute {
        /// Device to modify.
        device_key: DeviceKey,
        /// New mute state.
        muted: bool,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set stream volume
    SetStreamVolume {
        /// Stream to modify.
        stream_key: StreamKey,
        /// New volume level.
        volume: Volume,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set stream mute state
    SetStreamMute {
        /// Stream to modify.
        stream_key: StreamKey,
        /// New mute state.
        muted: bool,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set default input device
    SetDefaultInput {
        /// Device to set as default.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set default output device
    SetDefaultOutput {
        /// Device to set as default.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Move stream to different device
    MoveStream {
        /// Stream to move.
        stream_key: StreamKey,
        /// Target device.
        device_key: DeviceKey,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
    /// Set device port
    SetPort {
        /// Device to modify.
        device_key: DeviceKey,
        /// Port name to activate.
        port: String,
        /// Channel to send response.
        responder: oneshot::Sender<Result<(), Error>>,
    },
}
