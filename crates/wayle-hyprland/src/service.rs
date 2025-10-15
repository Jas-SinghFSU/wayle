use std::sync::Arc;

use tokio::sync::broadcast::{self, Sender};
use tokio_util::sync::CancellationToken;
use wayle_common::Property;

use crate::{
    HyprlandEvent, Result,
    core::{client::Client, layer::Layer, monitor::Monitor, workspace::Workspace},
    discovery::HyprlandDiscovery,
    ipc::{
        HyprMessenger,
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

    pub workspaces: Property<Vec<Arc<Workspace>>>,
    pub clients: Property<Vec<Arc<Client>>>,
    pub monitors: Property<Vec<Arc<Monitor>>>,
    pub layers: Property<Vec<Layer>>,
}

impl HyprlandService {
    /// Creates a new Hyprland service instance.
    ///
    /// Establishes connection to Hyprland's IPC sockets and initializes
    /// state by querying current monitors, workspaces, and windows.
    pub async fn new() -> Result<Self> {
        let (internal_tx, _) = broadcast::channel(100);
        let (hyprland_tx, _) = broadcast::channel(100);

        let cancellation_token = CancellationToken::new();
        let hypr_messenger = HyprMessenger::new()?;

        events::subscribe(internal_tx.clone(), hyprland_tx.clone()).await?;

        let mut internal_rx = internal_tx.subscribe();
        let mut hyprland_rx = hyprland_tx.subscribe();

        tokio::spawn(async move {
            while let Ok(event) = internal_rx.recv().await {
                println!("INTERNAL: {event:#?}");
            }
        });

        tokio::spawn(async move {
            while let Ok(event) = hyprland_rx.recv().await {
                println!("HYPRLAND: {event:#?}");
            }
        });

        let HyprlandDiscovery {
            workspaces,
            clients,
            monitors,
            layers,
        } = HyprlandDiscovery::new(hypr_messenger.clone(), &cancellation_token).await;

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
}
