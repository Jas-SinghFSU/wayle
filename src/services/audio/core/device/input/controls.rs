use tokio::sync::oneshot;
use tracing::instrument;

use crate::services::audio::{
    backend::{commands::Command, types::CommandSender},
    error::Error,
    types::device::DeviceKey,
    volume::types::Volume,
};

/// Controller for input device operations.
///
/// Provides stateless methods to control input devices through the backend.
pub(crate) struct InputDeviceController;

impl InputDeviceController {
    /// Set the volume for an input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    #[instrument(skip(command_tx), fields(device = ?device_key, volume = ?volume), err)]
    pub async fn set_volume(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        volume: Volume,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetVolume {
                device_key,
                volume,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }

    /// Set the mute state for an input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    #[instrument(skip(command_tx), fields(device = ?device_key, muted = muted), err)]
    pub async fn set_mute(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        muted: bool,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetMute {
                device_key,
                muted,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }

    /// Set the active port for an input device.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    #[instrument(skip(command_tx), fields(device = ?device_key, port = %port), err)]
    pub async fn set_port(
        command_tx: &CommandSender,
        device_key: DeviceKey,
        port: String,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetPort {
                device_key,
                port,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }

    /// Set a device as the default input.
    ///
    /// # Errors
    /// Returns error if backend communication fails or device operation fails.
    #[instrument(skip(command_tx), fields(device = ?device_key), err)]
    pub async fn set_as_default(
        command_tx: &CommandSender,
        device_key: DeviceKey,
    ) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();

        command_tx
            .send(Command::SetDefaultInput {
                device_key,
                responder: tx,
            })
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?;

        rx.await
            .map_err(|e| Error::CommandChannelDisconnected(e.to_string()))?
    }
}
