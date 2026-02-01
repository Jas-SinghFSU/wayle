use tokio::sync::broadcast::Sender;

use super::types::ServiceNotification;
use crate::{Address, Error, HyprlandEvent, Result};

pub(crate) fn handle_active_window(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((class, title)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            field: "window_data",
            expected: "comma-separated class,title",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveWindow {
        class: class.to_string(),
        title: title.to_string(),
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
        .send(ServiceNotification::ClientUpdated(address))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_open_window(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((address, rest)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "window_data",
            expected: "address,workspace,class,title",
            value: data.to_string(),
        });
    };
    let Some((workspace, rest)) = rest.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "window_data",
            expected: "address,workspace,class,title",
            value: data.to_string(),
        });
    };
    let Some((class, title)) = rest.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "window_data",
            expected: "address,workspace,class,title",
            value: data.to_string(),
        });
    };

    let address = Address::new(address.to_string());

    hyprland_tx.send(HyprlandEvent::OpenWindow {
        address: address.clone(),
        workspace: workspace.to_string(),
        class: class.to_string(),
        title: title.to_string(),
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
            field: "window_data",
            expected: "comma-separated address,workspace",
            value: data.to_string(),
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
    let event_data = format!("{event}>>{data}");
    let parts: Vec<&str> = data.split(',').collect();
    let [address, workspace_id, workspace] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data,
            field: "window_data",
            expected: "3 comma-separated values (address,workspace_id,workspace)",
            value: data.to_string(),
        });
    };
    let workspace_id = workspace_id.parse().map_err(|_| Error::EventParseError {
        event_data,
        field: "workspace_id",
        expected: "integer",
        value: (*workspace_id).to_string(),
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
    let event_data = format!("{event}>>{data}");
    let Some((address, floating)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "floating_mode_data",
            expected: "comma-separated address,floating",
            value: data.to_string(),
        });
    };
    let floating = match floating {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data,
                field: "floating",
                expected: "0 or 1",
                value: floating.to_string(),
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
            field: "window_title_data",
            expected: "comma-separated address,title",
            value: data.to_string(),
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
    let event_data = format!("{event}>>{data}");
    let Some((state, addresses_str)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "toggle_group_data",
            expected: "comma-separated state,addresses",
            value: data.to_string(),
        });
    };
    let state = match state {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data,
                field: "state",
                expected: "0 or 1",
                value: state.to_string(),
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
    let event_data = format!("{event}>>{data}");
    let Some((address, pinned)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "pin_data",
            expected: "comma-separated address,pinned",
            value: data.to_string(),
        });
    };
    let pinned = match pinned {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data,
                field: "pinned",
                expected: "0 or 1",
                value: pinned.to_string(),
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
    let event_data = format!("{event}>>{data}");
    let Some((address, minimized)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "minimized_data",
            expected: "comma-separated address,minimized",
            value: data.to_string(),
        });
    };
    let minimized = match minimized {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data,
                field: "minimized",
                expected: "0 or 1",
                value: minimized.to_string(),
            });
        }
    };

    hyprland_tx.send(HyprlandEvent::Minimized {
        address: Address::new(address.to_string()),
        minimized,
    })?;

    Ok(())
}
