use tokio::sync::broadcast::{self, Sender};

use crate::{HyprlandEvent, Result, ServiceNotification, events};

/// Hyprland compositor service providing reactive state and event streaming.
///
/// Connects to Hyprland's IPC sockets to query current state and receive events
/// about workspace changes, window lifecycle, monitor configuration, and more.
/// State is exposed through reactive properties that automatically update when
/// Hyprland emits relevant events.
pub struct HyprlandService {
    pub(crate) internal_tx: Sender<ServiceNotification>,
    pub(crate) hyprland_tx: Sender<HyprlandEvent>,
}

impl HyprlandService {
    /// Creates a new Hyprland service instance.
    ///
    /// Establishes connection to Hyprland's IPC sockets and initializes
    /// state by querying current monitors, workspaces, and windows.
    pub async fn new() -> Result<Self> {
        let (internal_tx, _) = broadcast::channel(100);
        let (hyprland_tx, _) = broadcast::channel(100);

        events::subscribe(internal_tx.clone(), hyprland_tx.clone()).await?;

        let mut hyprland_rx = hyprland_tx.subscribe();

        while let Ok(event) = hyprland_rx.recv().await {
            println!("{event:#?}");
        }

        Ok(Self {
            internal_tx,
            hyprland_tx,
        })
    }
}
