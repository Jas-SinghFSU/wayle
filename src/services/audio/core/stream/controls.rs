use tokio::sync::oneshot;

use crate::services::audio::{
    backend::{commands::Command, types::CommandSender},
    error::Error,
    types::{device::DeviceKey, stream::StreamKey},
    volume::types::Volume,
};

/// Controller for audio stream operations.
///
/// Provides stateless methods to control audio streams through the backend.
pub(crate) struct AudioStreamController;

impl AudioStreamController {
    pub async fn set_volume(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        volume: Volume,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetStreamVolume {
                stream_key,
                volume,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }

    /// Set the mute state for an audio stream.
    ///
    /// # Errors
    /// Returns error if backend communication fails or stream operation fails.
    pub async fn set_mute(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        muted: bool,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetStreamMute {
                stream_key,
                muted,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }

    /// Move a stream to a different device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or stream operation fails.
    pub async fn move_to_device(
        command_tx: &CommandSender,
        stream_key: StreamKey,
        device_key: DeviceKey,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::MoveStream {
                stream_key,
                device_key,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }
}
