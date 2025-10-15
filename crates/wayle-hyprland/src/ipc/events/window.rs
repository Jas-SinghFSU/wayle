use tokio::sync::broadcast::Sender;

use super::types::ServiceNotification;
use crate::{Address, Error, HyprlandEvent, Result};

pub(crate) fn handle_active_window(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let window_data: Vec<&str> = data.split(",").collect();
    let [class, title] = window_data.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected 2 comma-separated values (windowclass,windowtitle)".to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveWindow {
        class: (*class).to_string(),
        title: (*title).to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_active_window_v2(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::ActiveWindowV2 {
        address: address.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::ActiveClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_open_window(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let parts: Vec<&str> = data.split(',').collect();
    let [address, workspace, class, title] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected 4 comma-separated values (address,workspace,class,title)".to_string(),
        });
    };

    let address = Address::new((*address).to_string());

    hyprland_tx.send(HyprlandEvent::OpenWindow {
        address: address.clone(),
        workspace: (*workspace).to_string(),
        class: (*class).to_string(),
        title: (*title).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientCreated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_close_window(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::CloseWindow {
        address: address.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientRemoved(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_move_window(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((address, workspace)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated address,workspace".to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::MoveWindow {
        address: Address::new(address.to_string()),
        workspace: workspace.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_move_window_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let parts: Vec<&str> = data.split(',').collect();
    let [address, workspace_id, workspace] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected 3 comma-separated values (address,workspace_id,workspace)"
                .to_string(),
        });
    };
    let workspace_id = workspace_id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {workspace_id}"),
    })?;

    let address = Address::new((*address).to_string());
    hyprland_tx.send(HyprlandEvent::MoveWindowV2 {
        address: address.clone(),
        workspace_id,
        workspace: (*workspace).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientMoved(address, workspace_id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_change_floating_mode(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((address, floating)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated address,floating".to_string(),
        });
    };
    let floating = match floating {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid floating value: {floating}"),
            });
        }
    };

    let address = Address::new(address.to_string());
    hyprland_tx.send(HyprlandEvent::ChangeFloatingMode {
        address: address.clone(),
        floating,
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_urgent(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::Urgent { address })?;

    Ok(())
}

pub(crate) fn handle_window_title(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::WindowTitle {
        address: address.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_window_title_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((address, title)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated address,title".to_string(),
        });
    };

    let address = Address::new(address.to_string());
    hyprland_tx.send(HyprlandEvent::WindowTitleV2 {
        address: address.clone(),
        title: title.to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_toggle_group(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((state, addresses_str)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated state,addresses".to_string(),
        });
    };
    let state = match state {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid state value: {state}"),
            });
        }
    };
    let addresses: Vec<Address> = addresses_str
        .split(',')
        .map(|addr| Address::new(addr.to_string()))
        .collect();

    hyprland_tx.send(HyprlandEvent::ToggleGroup {
        state,
        addresses: addresses.clone(),
    })?;

    for addr in addresses {
        internal_tx
            .send(ServiceNotification::ClientUpdated(addr))
            .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;
    }

    Ok(())
}

pub(crate) fn handle_move_into_group(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::MoveIntoGroup {
        address: address.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_move_out_of_group(
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = Address::new(data.to_string());
    hyprland_tx.send(HyprlandEvent::MoveOutOfGroup {
        address: address.clone(),
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_pin(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((address, pinned)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated address,pinned".to_string(),
        });
    };
    let pinned = match pinned {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid pinned value: {pinned}"),
            });
        }
    };

    let address = Address::new(address.to_string());
    hyprland_tx.send(HyprlandEvent::Pin {
        address: address.clone(),
        pinned,
    })?;

    internal_tx
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_minimized(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((address, minimized)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated address,minimized".to_string(),
        });
    };
    let minimized = match minimized {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid minimized value: {minimized}"),
            });
        }
    };

    hyprland_tx.send(HyprlandEvent::Minimized {
        address: Address::new(address.to_string()),
        minimized,
    })?;

    Ok(())
}
