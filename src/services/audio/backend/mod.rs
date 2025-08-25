/// PulseAudio command handling  
pub mod command_handler;
/// Command definitions
pub mod commands;
/// Data conversion utilities
pub mod conversion;
/// Event subscription and handling
mod events;
/// Type definitions and aliases
pub mod types;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use command_handler::{handle_external_command, handle_internal_command};
pub use commands::Command;
pub use conversion::{convert_volume_from_pulse, convert_volume_to_pulse};
use libpulse_binding::context::{Context, FlagSet as ContextFlags};
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
pub use types::{
    DefaultDevice, DeviceListSender, DeviceStore, EventSender, ExternalCommand, InternalCommand,
    ServerInfo, StreamListSender, StreamStore,
};

use crate::services::{AudioError, AudioEvent, audio::tokio_mainloop::TokioMain};

/// Channel sender for backend commands
pub type CommandSender = mpsc::UnboundedSender<Command>;

/// Channel receiver for backend commands
pub type CommandReceiver = mpsc::UnboundedReceiver<Command>;

/// Channel receiver for backend events
pub type EventReceiver = broadcast::Receiver<AudioEvent>;

struct BackendState {
    devices: DeviceStore,
    streams: StreamStore,
    default_input: DefaultDevice,
    default_output: DefaultDevice,
}

impl BackendState {
    fn new() -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            streams: Arc::new(RwLock::new(HashMap::new())),
            default_input: Arc::new(RwLock::new(None)),
            default_output: Arc::new(RwLock::new(None)),
        }
    }
}

/// PulseAudio backend implementation
pub struct PulseBackend {
    state: BackendState,
    mainloop: TokioMain,
    context: Context,
}

impl PulseBackend {
    /// Start the PulseAudio backend task
    ///
    /// Creates a background task that monitors PulseAudio and processes commands.
    ///
    /// # Errors
    /// Returns error if PulseAudio connection fails
    pub async fn start(
        command_rx: CommandReceiver,
        event_tx: EventSender,
        cancellation_token: CancellationToken,
    ) -> Result<(), AudioError> {
        tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Handle::current();

            runtime.block_on(async move {
                info!("Starting PulseAudio backend");

                match Self::new().await {
                    Ok(backend) => {
                        if let Err(e) = backend.run(command_rx, event_tx, cancellation_token).await
                        {
                            error!("PulseAudio backend runtime error: {e}");
                        }
                    }
                    Err(e) => {
                        error!("Failed to create PulseAudio backend: {e}");
                    }
                }
            });
        });

        Ok(())
    }

    async fn new() -> Result<Self, AudioError> {
        let mut mainloop = TokioMain::new();
        info!("Creating PulseAudio context");
        let mut context = Context::new(&mainloop, "wayle-pulse")
            .ok_or_else(|| AudioError::ConnectionFailed("Failed to create context".to_string()))?;

        info!("Connecting to PulseAudio server");
        context
            .connect(None, ContextFlags::NOFLAGS, None)
            .map_err(|e| AudioError::ConnectionFailed(format!("Connection failed: {e}")))?;

        info!("Waiting for PulseAudio context to become ready");
        mainloop.wait_for_ready(&context).await.map_err(|e| {
            AudioError::ConnectionFailed(format!("Context failed to become ready: {e:?}"))
        })?;

        Ok(Self {
            state: BackendState::new(),
            mainloop,
            context,
        })
    }

    fn setup_event_monitoring(
        &mut self,
        event_tx: EventSender,
        cancellation_token: CancellationToken,
    ) -> Result<
        (
            mpsc::UnboundedSender<InternalCommand>,
            mpsc::UnboundedReceiver<InternalCommand>,
        ),
        AudioError,
    > {
        let (internal_command_tx, internal_command_rx) =
            mpsc::unbounded_channel::<InternalCommand>();

        info!("Setting up PulseAudio event subscription");
        events::start_event_processor(
            &mut self.context,
            self.state.devices.clone(),
            self.state.streams.clone(),
            event_tx,
            internal_command_tx.clone(),
            cancellation_token,
        )?;

        info!("Triggering initial device and stream discovery");
        let _ = internal_command_tx.send(InternalCommand::RefreshDevices);
        let _ = internal_command_tx.send(InternalCommand::RefreshStreams);
        let _ = internal_command_tx.send(InternalCommand::RefreshServerInfo);

        Ok((internal_command_tx, internal_command_rx))
    }

    fn spawn_command_processor(
        &self,
        mut command_rx: CommandReceiver,
        external_tx: mpsc::UnboundedSender<ExternalCommand>,
        cancellation_token: CancellationToken,
    ) -> tokio::task::JoinHandle<()> {
        let devices = self.state.devices.clone();
        let streams = self.state.streams.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("PulseBackend command handler cancelled");
                        return;
                    }
                    command = command_rx.recv() => {
                        let Some(command) = command else {
                            info!("Command channel closed");
                            return;
                        };

                        if matches!(command, Command::Shutdown) {
                            info!("Received shutdown command");
                            return;
                        }

                        Self::handle_command(command, &devices, &streams, &external_tx);
                    }
                }
            }
        })
    }

    #[allow(clippy::too_many_lines)]
    fn handle_command(
        command: Command,
        devices: &DeviceStore,
        streams: &StreamStore,
        external_tx: &mpsc::UnboundedSender<ExternalCommand>,
    ) {
        match command {
            Command::Shutdown => unreachable!("Shutdown handled in loop"),
            Command::GetDevice {
                device_key,
                responder,
            } => {
                let result = if let Ok(devices_guard) = devices.read() {
                    devices_guard
                        .values()
                        .find(|d| d.key() == device_key)
                        .cloned()
                        .ok_or(AudioError::DeviceNotFound(
                            device_key.index,
                            device_key.device_type,
                        ))
                } else {
                    Err(AudioError::BackendCommunicationFailed)
                };
                let _ = responder.send(result);
            }
            Command::GetStream {
                stream_key,
                responder,
            } => {
                let result = if let Ok(streams_guard) = streams.read() {
                    streams_guard
                        .values()
                        .find(|s| s.key() == stream_key)
                        .cloned()
                        .ok_or(AudioError::StreamNotFound(
                            stream_key.index,
                            stream_key.stream_type,
                        ))
                } else {
                    Err(AudioError::BackendCommunicationFailed)
                };
                let _ = responder.send(result);
            }
            Command::ListDevices { responder } => {
                let result = if let Ok(devices_guard) = devices.read() {
                    Ok(devices_guard.values().cloned().collect())
                } else {
                    Err(AudioError::BackendCommunicationFailed)
                };
                let _ = responder.send(result);
            }
            Command::ListStreams { responder } => {
                let result = if let Ok(streams_guard) = streams.read() {
                    Ok(streams_guard.values().cloned().collect())
                } else {
                    Err(AudioError::BackendCommunicationFailed)
                };
                let _ = responder.send(result);
            }
            Command::SetVolume {
                device_key,
                volume,
                responder,
            } => {
                let pulse_volume = convert_volume_to_pulse(&volume);
                let _ = external_tx.send(ExternalCommand::SetDeviceVolume {
                    device_key,
                    volume: pulse_volume,
                });
                let _ = responder.send(Ok(()));
            }
            Command::SetMute {
                device_key,
                muted,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::SetDeviceMute { device_key, muted });
                let _ = responder.send(Ok(()));
            }
            Command::SetStreamVolume {
                stream_key,
                volume,
                responder,
            } => {
                let pulse_volume = convert_volume_to_pulse(&volume);
                let _ = external_tx.send(ExternalCommand::SetStreamVolume {
                    stream_key,
                    volume: pulse_volume,
                });
                let _ = responder.send(Ok(()));
            }
            Command::SetStreamMute {
                stream_key,
                muted,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::SetStreamMute { stream_key, muted });
                let _ = responder.send(Ok(()));
            }
            Command::SetDefaultInput {
                device_key,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::SetDefaultInput { device_key });
                let _ = responder.send(Ok(()));
            }
            Command::SetDefaultOutput {
                device_key,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::SetDefaultOutput { device_key });
                let _ = responder.send(Ok(()));
            }
            Command::MoveStream {
                stream_key,
                device_key,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::MoveStream {
                    stream_key,
                    device_key,
                });
                let _ = responder.send(Ok(()));
            }
            Command::SetPort {
                device_key,
                port,
                responder,
            } => {
                let _ = external_tx.send(ExternalCommand::SetPort { device_key, port });
                let _ = responder.send(Ok(()));
            }
        }
    }

    fn spawn_context_handler(
        self,
        mut internal_command_rx: mpsc::UnboundedReceiver<InternalCommand>,
        mut external_rx: mpsc::UnboundedReceiver<ExternalCommand>,
        event_tx: EventSender,
        cancellation_token: CancellationToken,
    ) -> (TokioMain, tokio::task::JoinHandle<()>) {
        let mut context = self.context;
        let state = self.state;

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        info!("PulseBackend context handler cancelled");
                        context.disconnect();
                        return;
                    }
                    Some(command) = internal_command_rx.recv() => {
                        handle_internal_command(
                            &mut context,
                            command,
                            &state.devices,
                            &state.streams,
                            &event_tx,
                            &state.default_input,
                            &state.default_output,
                        );
                    }
                    Some(command) = external_rx.recv() => {
                        handle_external_command(&mut context, command, &state.devices, &state.streams);
                    }
                    else => {
                        info!("Internal command channel closed");
                        return;
                    }
                }
            }
        });

        (self.mainloop, handle)
    }

    async fn run(
        mut self,
        command_rx: CommandReceiver,
        event_tx: EventSender,
        cancellation_token: CancellationToken,
    ) -> Result<(), AudioError> {
        let (_, internal_command_rx) =
            self.setup_event_monitoring(event_tx.clone(), cancellation_token.child_token())?;

        let (external_tx, external_rx) = mpsc::unbounded_channel::<ExternalCommand>();

        info!("PulseAudio backend fully initialized and monitoring");

        let command_handle =
            self.spawn_command_processor(command_rx, external_tx, cancellation_token.child_token());

        let (mut mainloop, context_handle) = self.spawn_context_handler(
            internal_command_rx,
            external_rx,
            event_tx.clone(),
            cancellation_token.child_token(),
        );

        tokio::select! {
            _ = mainloop.run() => {
                info!("PulseAudio mainloop exited");
            }
            _ = command_handle => {
                info!("Command processing loop exited");
            }
            _ = context_handle => {
                info!("Context handling loop exited");
            }
            _ = cancellation_token.cancelled() => {
                info!("PulseAudio backend cancelled");
            }
        }

        info!("PulseAudio backend stopped");
        Ok(())
    }
}
