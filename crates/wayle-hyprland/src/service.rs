use std::{sync::Arc, time::Duration};

use futures::Stream;
use tokio::sync::broadcast::{self, Sender};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tokio_util::sync::CancellationToken;
use tracing::{error, instrument};
use wayle_common::Property;

use crate::{
    Address, BindData, CursorPosition, DeviceInfo, HyprlandEvent, Result, WorkspaceId,
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    discovery::HyprlandDiscovery,
    ipc::{
        DismissProps, HyprMessenger, OutputCommand, SetErrorCommand,
        events::{self, types::ServiceNotification},
    },
};

/// Hyprland compositor service providing reactive state and event streaming.
///
/// Connects to Hyprland's IPC sockets to query current state and receive events
/// about workspace changes, window lifecycle, monitor configuration, and more.
/// State is exposed through reactive properties that automatically update when
/// Hyprland emits relevant events.
pub struct HyprlandService {
    pub(crate) internal_tx: Sender<ServiceNotification>,
    pub(crate) hyprland_tx: Sender<HyprlandEvent>,
    pub(crate) cancellation_token: CancellationToken,
    pub(crate) hypr_messenger: HyprMessenger,

    /// Reactive property containing all workspaces in Hyprland.
    pub workspaces: Property<Vec<Arc<Workspace>>>,
    /// Reactive property containing all active client windows.
    pub clients: Property<Vec<Arc<Client>>>,
    /// Reactive property containing all connected monitors.
    pub monitors: Property<Vec<Arc<Monitor>>>,
    /// Reactive property containing all layer shell surfaces.
    pub layers: Property<Vec<Layer>>,
}

impl HyprlandService {
    /// Creates a new Hyprland service instance.
    ///
    /// # Errors
    /// Returns error if IPC socket connection fails or initial state query fails.
    #[instrument(err)]
    pub async fn new() -> Result<Self> {
        let (internal_tx, _) = broadcast::channel(100);
        let (hyprland_tx, _) = broadcast::channel(100);

        let cancellation_token = CancellationToken::new();
        let hypr_messenger = HyprMessenger::new()?;

        events::subscribe(internal_tx.clone(), hyprland_tx.clone()).await?;

        let HyprlandDiscovery {
            workspaces,
            clients,
            monitors,
            layers,
        } = HyprlandDiscovery::new(hypr_messenger.clone(), &internal_tx, &cancellation_token).await;

        Ok(Self {
            internal_tx,
            hyprland_tx,
            cancellation_token,
            hypr_messenger,
            workspaces: Property::new(workspaces),
            clients: Property::new(clients),
            monitors: Property::new(monitors),
            layers: Property::new(layers),
        })
    }

    /// Returns a client window by its address if it exists.
    #[instrument(skip(self), fields(address = %address))]
    pub async fn client(&self, address: Address) -> Option<Arc<Client>> {
        self.clients
            .get()
            .into_iter()
            .find(|client| client.address.get() == address)
    }

    /// Returns a workspace by its ID if it exists.
    #[instrument(skip(self), fields(id = %id))]
    pub async fn workspace(&self, id: WorkspaceId) -> Option<Arc<Workspace>> {
        self.workspaces
            .get()
            .into_iter()
            .find(|workspace| workspace.id.get() == id)
    }

    /// Returns a monitor by its name if it exists.
    #[instrument(skip(self), fields(name = %name))]
    pub async fn monitor(&self, name: String) -> Option<Arc<Monitor>> {
        self.monitors
            .get()
            .into_iter()
            .find(|monitor| monitor.name.get() == name)
    }

    /// Returns a layer shell surface by its address if it exists.
    #[instrument(skip(self), fields(address = %address))]
    pub async fn layer(&self, address: Address) -> Option<Layer> {
        self.layers
            .get()
            .into_iter()
            .find(|layer| layer.address.get() == address)
    }

    /// Returns a stream of Hyprland events.
    ///
    /// The stream emits events for workspace changes, window lifecycle,
    /// monitor configuration, and other compositor events.
    pub fn events(&self) -> impl Stream<Item = HyprlandEvent> {
        let hyprland_rx = self.hyprland_tx.subscribe();

        BroadcastStream::new(hyprland_rx).filter_map(|result| result.ok())
    }

    /// Executes a Hyprland dispatcher command.
    ///
    /// # Errors
    /// Returns error if the command fails or IPC communication fails.
    #[instrument(skip(self), fields(command = %command), err)]
    pub async fn dispatch(&self, command: &str) -> Result<String> {
        self.hypr_messenger.dispatch(command).await
    }

    /// Sets a Hyprland configuration keyword at runtime.
    ///
    /// # Errors
    /// Returns error if the keyword is invalid or IPC communication fails.
    #[instrument(skip(self), fields(command = %command), err)]
    pub async fn keyword(&self, command: &str) -> Result<String> {
        self.hypr_messenger.keyword(command).await
    }

    /// Reloads Hyprland configuration.
    ///
    /// # Errors
    /// Returns error if reload fails or IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn reload(&self) -> Result<String> {
        self.hypr_messenger.reload().await
    }

    /// Kills the active window.
    ///
    /// # Errors
    /// Returns error if operation fails or IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn kill(&self) -> Result<String> {
        self.hypr_messenger.kill().await
    }

    /// Sets the cursor theme and size.
    ///
    /// # Errors
    /// Returns error if the theme is invalid or IPC communication fails.
    #[instrument(skip(self), fields(theme = %theme, size = %size), err)]
    pub async fn set_cursor(&self, theme: &str, size: u8) -> Result<String> {
        self.hypr_messenger.set_cursor(theme, size).await
    }

    /// Manages output (monitor) creation and removal.
    ///
    /// # Errors
    /// Returns error if the output command fails or IPC communication fails.
    #[instrument(skip(self), fields(command = ?command), err)]
    pub async fn output(&self, command: OutputCommand<'_>) -> Result<String> {
        self.hypr_messenger.output(command).await
    }

    /// Switches keyboard layout for a specific device.
    ///
    /// # Errors
    /// Returns error if the device or layout is invalid, or IPC communication fails.
    #[instrument(skip(self), fields(device = %device, command = %command), err)]
    pub async fn switch_xkb_layout(&self, device: &str, command: &str) -> Result<String> {
        self.hypr_messenger.switch_xkb_layout(device, command).await
    }

    /// Sets or clears the error message displayed by Hyprland.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), fields(command = ?command), err)]
    pub async fn set_error(&self, command: SetErrorCommand<'_>) -> Result<String> {
        self.hypr_messenger.set_error(command).await
    }

    /// Sends a notification using Hyprland's notification system.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(
        skip(self, message),
        fields(
            icon = ?icon,
            time_ms = %time.as_millis(),
            color = ?color
        ),
        err
    )]
    pub async fn notify(
        &self,
        icon: Option<&str>,
        time: Duration,
        color: Option<&str>,
        message: &str,
    ) -> Result<String> {
        self.hypr_messenger.notify(icon, time, color, message).await
    }

    /// Dismisses notifications from Hyprland's notification system.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), fields(props = ?props), err)]
    pub async fn dismiss_notify(&self, props: DismissProps) -> Result<String> {
        self.hypr_messenger.dismiss_notify(props).await
    }

    /// Gets a property value from a window.
    ///
    /// # Errors
    /// Returns error if the window or property is invalid, or IPC communication fails.
    #[instrument(skip(self), fields(window = %window, property = %property), err)]
    pub async fn get_prop(&self, window: &str, property: &str) -> Result<String> {
        self.hypr_messenger.get_prop(window, property).await
    }

    /// Returns the currently active workspace if it exists.
    #[instrument(skip(self))]
    pub async fn active_workspace(&self) -> Option<Arc<Workspace>> {
        let workspace = match self.hypr_messenger.active_workspace().await {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to get active workspace: {e}");
                return None;
            }
        };

        self.workspaces
            .get()
            .into_iter()
            .find(|w| w.id.get() == workspace.id)
    }

    /// Returns the currently focused window if it exists.
    #[instrument(skip(self))]
    pub async fn active_window(&self) -> Option<Arc<Client>> {
        let window = match self.hypr_messenger.active_window().await {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to get active workspace: {e}");
                return None;
            }
        };

        self.clients
            .get()
            .into_iter()
            .find(|w| w.address.get() == window.address)
    }

    /// Returns the Hyprland version string.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn version(&self) -> Result<String> {
        self.hypr_messenger.version().await
    }

    /// Returns the current cursor position.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn cursor_pos(&self) -> Result<CursorPosition> {
        self.hypr_messenger.cursor_pos().await
    }

    /// Returns all configured keybinds.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn binds(&self) -> Result<Vec<BindData>> {
        self.hypr_messenger.binds().await
    }

    /// Returns information about all input devices.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn devices(&self) -> Result<DeviceInfo> {
        self.hypr_messenger.devices().await
    }

    /// Returns all available keyboard layouts.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn layouts(&self) -> Result<Vec<String>> {
        self.hypr_messenger.layouts().await
    }

    /// Returns the currently active submap name.
    ///
    /// # Errors
    /// Returns error if IPC communication fails.
    #[instrument(skip(self), err)]
    pub async fn submap(&self) -> Result<String> {
        self.hypr_messenger.submap().await
    }
}

impl Drop for HyprlandService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
