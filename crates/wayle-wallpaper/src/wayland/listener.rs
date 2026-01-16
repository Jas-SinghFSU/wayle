use std::{
    sync::mpsc as std_mpsc,
    thread::{self, JoinHandle},
};

use smithay_client_toolkit::{
    delegate_output, delegate_registry,
    output::{OutputHandler, OutputState},
    reexports::client::{
        Connection, QueueHandle,
        globals::{GlobalList, registry_queue_init},
        protocol::wl_output::WlOutput,
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};
use tokio::sync::mpsc as tokio_mpsc;
use tracing::{info, warn};

/// Events emitted when monitors connect or disconnect.
#[derive(Debug, Clone)]
pub enum OutputEvent {
    Added(String),
    Removed(String),
}

/// Watches for Wayland output changes in the background.
pub struct OutputWatcher {
    event_rx: tokio_mpsc::UnboundedReceiver<OutputEvent>,
    shutdown_tx: std_mpsc::Sender<()>,
    thread_handle: Option<JoinHandle<()>>,
}

impl OutputWatcher {
    /// Starts watching for output changes.
    ///
    /// Returns `None` if Wayland is unavailable.
    pub fn start() -> Option<Self> {
        let conn = match Connection::connect_to_env() {
            Ok(conn) => conn,
            Err(err) => {
                warn!(error = %err, "cannot connect to Wayland display");
                return None;
            }
        };

        let (event_tx, event_rx) = tokio_mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = std_mpsc::channel();

        let thread_handle = match thread::Builder::new()
            .name("wayland-output-watcher".into())
            .spawn(move || {
                run_event_loop(conn, event_tx, shutdown_rx);
            }) {
            Ok(handle) => handle,
            Err(err) => {
                warn!(error = %err, "cannot spawn Wayland output watcher thread");
                return None;
            }
        };

        Some(Self {
            event_rx,
            shutdown_tx,
            thread_handle: Some(thread_handle),
        })
    }

    /// Returns the receiver for output events.
    pub fn events(&mut self) -> &mut tokio_mpsc::UnboundedReceiver<OutputEvent> {
        &mut self.event_rx
    }

    /// Queries all currently connected outputs.
    #[allow(clippy::cognitive_complexity)]
    pub fn query_outputs() -> Option<Vec<String>> {
        let conn = match Connection::connect_to_env() {
            Ok(conn) => conn,
            Err(err) => {
                warn!(error = %err, "cannot connect to Wayland for output query");
                return None;
            }
        };

        let (globals, mut event_queue) = match registry_queue_init(&conn) {
            Ok(result) => result,
            Err(err) => {
                warn!(error = %err, "cannot initialize Wayland registry");
                return None;
            }
        };

        let qh = event_queue.handle();
        let (tx, _rx) = tokio_mpsc::unbounded_channel();
        let mut state = WatcherState::new(&globals, &qh, tx);

        if let Err(err) = event_queue.roundtrip(&mut state) {
            warn!(error = %err, "wayland roundtrip failed during output query");
            return None;
        }
        if let Err(err) = event_queue.roundtrip(&mut state) {
            warn!(error = %err, "wayland roundtrip failed during output query");
            return None;
        }

        let outputs: Vec<String> = state
            .output_state
            .outputs()
            .filter_map(|output| {
                state
                    .output_state
                    .info(&output)
                    .and_then(|info| info.name.clone())
            })
            .collect();

        Some(outputs)
    }
}

impl Drop for OutputWatcher {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(());
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

struct WatcherState {
    registry_state: RegistryState,
    output_state: OutputState,
    event_tx: tokio_mpsc::UnboundedSender<OutputEvent>,
}

impl WatcherState {
    fn new(
        globals: &GlobalList,
        qh: &QueueHandle<Self>,
        event_tx: tokio_mpsc::UnboundedSender<OutputEvent>,
    ) -> Self {
        Self {
            registry_state: RegistryState::new(globals),
            output_state: OutputState::new(globals, qh),
            event_tx,
        }
    }
}

impl OutputHandler for WatcherState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, output: WlOutput) {
        if let Some(info) = self.output_state.info(&output)
            && let Some(name) = &info.name
        {
            info!(output = %name, "Output added");
            let _ = self.event_tx.send(OutputEvent::Added(name.clone()));
        }
    }

    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: WlOutput) {}

    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, output: WlOutput) {
        if let Some(info) = self.output_state.info(&output)
            && let Some(name) = &info.name
        {
            info!(output = %name, "Output removed");
            let _ = self.event_tx.send(OutputEvent::Removed(name.clone()));
        }
    }
}

impl ProvidesRegistryState for WatcherState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers![OutputState];
}

delegate_registry!(WatcherState);
delegate_output!(WatcherState);

#[allow(clippy::cognitive_complexity)]
fn run_event_loop(
    conn: Connection,
    event_tx: tokio_mpsc::UnboundedSender<OutputEvent>,
    shutdown_rx: std_mpsc::Receiver<()>,
) {
    let (globals, mut event_queue) = match registry_queue_init(&conn) {
        Ok(result) => result,
        Err(err) => {
            warn!(error = %err, "cannot initialize Wayland registry");
            return;
        }
    };

    let qh = event_queue.handle();
    let mut state = WatcherState::new(&globals, &qh, event_tx);

    if event_queue.roundtrip(&mut state).is_err() {
        return;
    }
    if event_queue.roundtrip(&mut state).is_err() {
        return;
    }

    for output in state.output_state.outputs() {
        if let Some(info) = state.output_state.info(&output)
            && let Some(name) = &info.name
        {
            info!(output = %name, "Output discovered");
            let _ = state.event_tx.send(OutputEvent::Added(name.clone()));
        }
    }

    loop {
        if shutdown_rx.try_recv().is_ok() {
            return;
        }

        match event_queue.blocking_dispatch(&mut state) {
            Ok(_) => {}
            Err(err) => {
                warn!(error = %err, "Wayland dispatch error");
                return;
            }
        }
    }
}
