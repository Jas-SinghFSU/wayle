use crate::{Address, MonitorId, Namespace, ScreencastOwner, WorkspaceId};

/// Internal events for our hyprland service
#[derive(Debug, Clone)]
pub(crate) enum ServiceNotification {
    WorkspaceCreated(WorkspaceId),
    WorkspaceUpdated(WorkspaceId),
    WorkspaceRemoved(WorkspaceId),
    WorkspaceFocused(WorkspaceId),
    WorkspaceMoved(WorkspaceId),

    MonitorCreated(String),
    MonitorUpdated(String),
    MonitorRemoved(String),

    ClientCreated(Address),
    ClientUpdated(Address),
    ClientRemoved(Address),
    ClientMoved(Address, WorkspaceId),

    LayerCreated(Namespace),
    LayerRemoved(Namespace),
}

/// Structured events emitted by Hyprland
#[derive(Debug, Clone)]
pub enum HyprlandEvent {
    /// Emitted on workspace change (v1).
    ///
    /// Is emitted ONLY when a user requests a workspace change, and is
    /// not emitted on mouse movements.
    Workspace {
        /// Workspace name.
        name: String,
    },

    /// Emitted on workspace change (v2).
    ///
    /// Is emitted ONLY when a user requests a workspace change, and is
    /// not emitted on mouse movements.
    WorkspaceV2 {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when the active monitor is changed (v1).
    FocusedMon {
        /// Monitor name.
        name: String,
        /// Workspace name.
        workspace: String,
    },

    /// Emitted when the active monitor is changed (v2).
    FocusedMonV2 {
        /// Monitor name.
        name: String,
        /// Active workspace ID on this monitor.
        workspace_id: WorkspaceId,
    },

    /// Emitted when the active window is changed (v1).
    ActiveWindow {
        /// Window class.
        class: String,
        /// Window title.
        title: String,
    },

    /// Emitted when the active window is changed (v2).
    ActiveWindowV2 {
        /// Window address.
        address: Address,
    },

    /// Emitted when a fullscreen status of a window changes.
    Fullscreen {
        /// Whether entering fullscreen (true) or exiting (false).
        fullscreen: bool,
    },

    /// Emitted when a monitor is removed (v1).
    MonitorRemoved {
        /// Monitor name.
        name: String,
    },

    /// Emitted when a monitor is removed (v2).
    MonitorRemovedV2 {
        /// Monitor ID.
        id: MonitorId,
        /// Monitor name.
        name: String,
        /// Monitor description.
        description: String,
    },

    /// Emitted when a monitor is added (v1).
    MonitorAdded {
        /// Monitor name.
        name: String,
    },

    /// Emitted when a monitor is added (v2).
    MonitorAddedV2 {
        /// Monitor ID.
        id: MonitorId,
        /// Monitor name.
        name: String,
        /// Monitor description.
        description: String,
    },

    /// Emitted when a workspace is created (v1).
    CreateWorkspace {
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is created (v2).
    CreateWorkspaceV2 {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is destroyed (v1).
    DestroyWorkspace {
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is destroyed (v2).
    DestroyWorkspaceV2 {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
    },

    /// Emitted when a workspace is moved to a different monitor (v1).
    MoveWorkspace {
        /// Workspace name.
        name: String,
        /// Target monitor name.
        monitor: String,
    },

    /// Emitted when a workspace is moved to a different monitor (v2).
    MoveWorkspaceV2 {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        name: String,
        /// Target monitor name.
        monitor: String,
    },

    /// Emitted when a workspace is renamed.
    RenameWorkspace {
        /// Workspace ID.
        id: WorkspaceId,
        /// New workspace name.
        new_name: String,
    },

    /// Emitted when the special workspace opened in a monitor changes (v1).
    ///
    /// Closing results in an empty workspace name.
    ActiveSpecial {
        /// Workspace name.
        workspace: String,
        /// Monitor name.
        monitor: String,
    },

    /// Emitted when the special workspace opened in a monitor changes (v2).
    ///
    /// Closing results in empty workspace ID and name values.
    ActiveSpecialV2 {
        /// Workspace ID.
        id: WorkspaceId,
        /// Workspace name.
        workspace: String,
        /// Monitor name.
        monitor: String,
    },

    /// Emitted on a layout change of the active keyboard.
    ActiveLayout {
        /// Keyboard name.
        keyboard: String,
        /// Layout name.
        layout: String,
    },

    /// Emitted when a window is opened.
    OpenWindow {
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
    CloseWindow {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window is moved to a workspace (v1).
    MoveWindow {
        /// Window address.
        address: Address,
        /// Target workspace name.
        workspace: String,
    },

    /// Emitted when a window is moved to a workspace (v2).
    MoveWindowV2 {
        /// Window address.
        address: Address,
        /// Target workspace ID.
        workspace_id: WorkspaceId,
        /// Target workspace name.
        workspace: String,
    },

    /// Emitted when a layer surface is mapped.
    OpenLayer {
        /// Layer namespace.
        namespace: String,
    },

    /// Emitted when a layer surface is unmapped.
    CloseLayer {
        /// Layer namespace.
        namespace: String,
    },

    /// Emitted when a keybind submap changes.
    ///
    /// Empty means default submap.
    Submap {
        /// Submap name.
        name: String,
    },

    /// Emitted when a window changes its floating mode.
    ChangeFloatingMode {
        /// Window address.
        address: Address,
        /// Whether the window is floating.
        floating: bool,
    },

    /// Emitted when a window requests an urgent state.
    Urgent {
        /// Window address.
        address: Address,
    },

    /// Emitted when a screencopy state of a client changes.
    ///
    /// Keep in mind there might be multiple separate clients.
    Screencast {
        /// Whether screencasting is active.
        state: bool,
        /// Whether it's a monitor or window share.
        owner: ScreencastOwner,
    },

    /// Emitted when a window title changes (v1).
    WindowTitle {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window title changes (v2).
    WindowTitleV2 {
        /// Window address.
        address: Address,
        /// New window title.
        title: String,
    },

    /// Emitted when the togglegroup command is used.
    ///
    /// Returns state and handle where the state is a toggle status and the
    /// handle is one or more window addresses separated by a comma.
    ToggleGroup {
        /// Whether a group was created (true) or destroyed (false).
        state: bool,
        /// Window addresses that were part of the group.
        addresses: Vec<Address>,
    },

    /// Emitted when a window is merged into a group.
    MoveIntoGroup {
        /// Window address.
        address: Address,
    },

    /// Emitted when a window is removed from a group.
    MoveOutOfGroup {
        /// Window address.
        address: Address,
    },

    /// Emitted when ignoregrouplock is toggled.
    IgnoreGroupLock {
        /// Whether group lock is ignored.
        ignore: bool,
    },

    /// Emitted when lockgroups is toggled.
    LockGroups {
        /// Whether groups are locked.
        locked: bool,
    },

    /// Emitted when the config is done reloading.
    ConfigReloaded,

    /// Emitted when a window is pinned or unpinned.
    Pin {
        /// Window address.
        address: Address,
        /// Whether the window is pinned.
        pinned: bool,
    },

    /// Emitted when an external taskbar-like app requests a window to be minimized.
    Minimized {
        /// Window address.
        address: Address,
        /// Whether the window is minimized.
        minimized: bool,
    },

    /// Emitted when an app requests to ring the system bell via xdg-system-bell-v1.
    ///
    /// Window address parameter may be empty.
    Bell {
        /// Window address (may be empty).
        address: Option<Address>,
    },
}
