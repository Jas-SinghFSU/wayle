use tokio::sync::broadcast::Sender;

use crate::{HyprlandEvent, Result};

pub(crate) fn handle_open_layer(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let namespace = data.to_string();
    hyprland_tx.send(HyprlandEvent::OpenLayer {
        namespace: namespace.clone(),
    })?;

    Ok(())
}

pub(crate) fn handle_close_layer(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let namespace = data.to_string();
    hyprland_tx.send(HyprlandEvent::CloseLayer {
        namespace: namespace.clone(),
    })?;

    Ok(())
}
