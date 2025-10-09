use tokio::sync::broadcast::Sender;

use crate::{HyprlandEvent, Result, ServiceNotification};

pub mod types;

pub(crate) fn handle_open_layer(
    _event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::OpenLayer {
        namespace: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_close_layer(
    _event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::CloseLayer {
        namespace: data.to_string(),
    })?;

    Ok(())
}
