use tokio::sync::oneshot;
use tracing::instrument;

use crate::{
    backend::{commands::Command, types::CommandSender},
    error::Error,
    types::device::DeviceKey,
    volume::types::Volume,
};

/// Controller for output device operations.
///
/// Provides stateless methods to control output devices through the backend.
pub(crate) struct OutputDeviceController;

impl OutputDeviceController {
    /// Set the volume for an output device.
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
            .map_err(|_| Error::CommandChannelDisconnected)?;

        rx.await.map_err(|_| Error::CommandChannelDisconnected)?
    }

    /// Set the mute state for an output device.
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
            .map_err(|_| Error::CommandChannelDisconnected)?;

        rx.await.map_err(|_| Error::CommandChannelDisconnected)?
    }

    /// Set the active port for an output device.
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
            .map_err(|_| Error::CommandChannelDisconnected)?;

        rx.await.map_err(|_| Error::CommandChannelDisconnected)?
    }

    /// Set a device as the default output.
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
            .send(Command::SetDefaultOutput {
                device_key,
                responder: tx,
            })
            .map_err(|_| Error::CommandChannelDisconnected)?;

        rx.await.map_err(|_| Error::CommandChannelDisconnected)?
    }
}
