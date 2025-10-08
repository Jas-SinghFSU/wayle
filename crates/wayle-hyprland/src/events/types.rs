use crate::{Address, MonitorId, ScreencastOwner, WorkspaceId};

/// Raw Hyprland IPC events parsed from the socket.
pub enum HyprlandEvent {}

/// Structured events emitted by Hyprland
pub enum ServiceNotification {
    /// Emitted when a workspace is created.
    WorkspaceCreated {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is destroyed.
    WorkspaceDestroyed {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when the active workspace changes.
    ///
    /// This is emitted only when a user requests a workspace change, and is
    /// not emitted on mouse movements.
    WorkspaceFocused {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is moved to a different monitor.
    WorkspaceMoved {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
        /// Target monitor name.
        monitor: String,
    },

    /// Emitted when a workspace is renamed.
    WorkspaceRenamed {
        /// Workspace ID.
        id: WorkspaceId,
        /// New workspace name.
        new_name: String,
    },

    /// Emitted when a monitor is added (connected).
    MonitorAdded {
        /// Monitor ID.
        id: MonitorId,
        /// Monitor name.
        name: String,
        /// Monitor description.
        description: String,
    },

    /// Emitted when a monitor is removed (disconnected).
    MonitorRemoved {
        /// Monitor ID.
        id: MonitorId,
        /// Monitor name.
        name: String,
        /// Monitor description.
        description: String,
    },

    /// Emitted when the active monitor is changed.
    MonitorFocused {
        /// Monitor name.
        name: String,
        /// Active workspace ID on this monitor.
        workspace_id: WorkspaceId,
    },

    /// Emitted when a window is opened.
    ClientAdded {
        /// Window address.
        address: Address,
        /// Workspace name.
        workspace: String,
        /// Window class.
        class: String,
        /// Window title.
        title: String,
    },

    /// Emitted when a window is closed.
    ClientRemoved {
        /// Window address.
        address: Address,
    },

    /// Emitted when the active window is changed.
    ClientFocused {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window is moved to a workspace.
    ClientMoved {
        /// Window address.
        address: Address,
        /// Target workspace ID.
        workspace_id: WorkspaceId,
        /// Target workspace name.
        workspace: String,
    },

    /// Emitted when a window changes its floating mode.
    ClientFloatingChanged {
        /// Window address.
        address: Address,
        /// Whether the window is floating.
        floating: bool,
    },

    /// Emitted when a fullscreen status of a window changes.
    ClientFullscreenChanged {
        /// Whether the window is fullscreen.
        fullscreen: bool,
    },

    /// Emitted when a window is pinned or unpinned.
    ClientPinnedChanged {
        /// Window address.
        address: Address,
        /// Whether the window is pinned.
        pinned: bool,
    },

    /// Emitted when an external taskbar-like app requests a window to be
    /// minimized.
    ClientMinimizedChanged {
        /// Window address.
        address: Address,
        /// Whether the window is minimized.
        minimized: bool,
    },

    /// Emitted when a window title changes.
    ClientTitleChanged {
        /// Window address.
        address: Address,
        /// New window title.
        title: String,
    },

    /// Emitted when a window requests an urgent state.
    ClientUrgent {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window is merged into a group.
    ClientMovedIntoGroup {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window is removed from a group.
    ClientMovedOutOfGroup {
        /// Window address.
        address: Address,
    },

    /// Emitted when a layer surface is mapped.
    LayerAdded {
        /// Layer namespace.
        namespace: String,
    },

    /// Emitted when a layer surface is unmapped.
    LayerRemoved {
        /// Layer namespace.
        namespace: String,
    },

    /// Emitted when the special workspace opened in a monitor changes.
    ///
    /// Closing the special workspace results in empty workspace ID and name
    /// values.
    SpecialWorkspaceChanged {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        workspace: String,
        /// Monitor name.
        monitor: String,
    },

    /// Emitted on a layout change of the active keyboard.
    KeyboardLayoutChanged {
        /// Keyboard name.
        keyboard: String,
        /// Layout name.
        layout: String,
    },

    /// Emitted when the togglegroup command is used.
    ///
    /// The state indicates whether a group was created (true) or destroyed
    /// (false), and addresses contains the window addresses that were part
    /// of the group.
    GroupToggled {
        /// Whether a group was created (true) or destroyed (false).
        state: bool,
        /// Window addresses that were part of the group.
        addresses: Vec<Address>,
    },

    /// Emitted when lockgroups is toggled.
    GroupLockChanged {
        /// Whether groups are locked.
        locked: bool,
    },

    /// Emitted when ignoregrouplock is toggled.
    IgnoreGroupLockChanged {
        /// Whether group lock is ignored.
        ignore: bool,
    },

    /// Emitted when the config is done reloading.
    ConfigReloaded,

    /// Emitted when a keybind submap changes.
    ///
    /// An empty submap name means the default submap.
    SubmapChanged {
        /// Submap name.
        name: String,
    },

    /// Emitted when a screencopy state of a client changes.
    ///
    /// Keep in mind there might be multiple separate clients. The owner
    /// indicates whether it's a monitor share or window share.
    ScreencastStateChanged {
        /// Whether screencasting is active.
        state: bool,
        /// Whether it's a monitor or window share.
        owner: ScreencastOwner,
    },

    /// Emitted when an app requests to ring the system bell.
    ///
    /// The window address parameter may be empty.
    BellRequested {
        /// Window address (may be empty).
        address: Option<Address>,
    },
}
