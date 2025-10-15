use tokio::sync::broadcast::Sender;

use super::types::ServiceNotification;
use crate::{Error, HyprlandEvent, Result};

pub(crate) fn handle_open_layer(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let namespace = data.to_string();
    hyprland_tx.send(HyprlandEvent::OpenLayer {
        namespace: namespace.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::LayerCreated(namespace))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_close_layer(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let namespace = data.to_string();
    hyprland_tx.send(HyprlandEvent::CloseLayer {
        namespace: namespace.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::LayerRemoved(namespace))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}
