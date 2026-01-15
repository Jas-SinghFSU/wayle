use tokio::sync::broadcast::Sender;

use super::types::ServiceNotification;
use crate::{Error, HyprlandEvent, Result};

pub(crate) fn handle_focused_mon(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let monitor_data: Vec<&str> = data.split(",").collect();
    let [name, workspace] = monitor_data.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            field: "monitor_data",
            expected: "2 comma-separated values (name,workspace)",
            value: data.to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::FocusedMon {
        name: (*name).to_string(),
        workspace: (*workspace).to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_focused_mon_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let Some((name, workspace_id)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data,
            field: "monitor_data",
            expected: "comma-separated name,workspace_id",
            value: data.to_string(),
        });
    };
    let workspace_id = workspace_id.parse().map_err(|_| Error::EventParseError {
        event_data,
        field: "workspace_id",
        expected: "integer",
        value: workspace_id.to_string(),
    })?;

    let monitor_name = name.to_string();
    hyprland_tx.send(HyprlandEvent::FocusedMonV2 {
        name: monitor_name.clone(),
        workspace_id,
    })?;

    internal_tx
        .send(ServiceNotification::MonitorUpdated(monitor_name))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_monitor_removed(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let monitor_name = data.to_string();
    hyprland_tx.send(HyprlandEvent::MonitorRemoved {
        name: monitor_name.clone(),
    })?;

    Ok(())
}

pub(crate) fn handle_monitor_removed_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let parts: Vec<&str> = data.split(',').collect();
    let [id, name, description] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data,
            field: "monitor_data",
            expected: "3 comma-separated values (id,name,description)",
            value: data.to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data,
        field: "monitor_id",
        expected: "integer",
        value: (*id).to_string(),
    })?;

    let monitor_name = (*name).to_string();
    hyprland_tx.send(HyprlandEvent::MonitorRemovedV2 {
        id,
        name: monitor_name.clone(),
        description: (*description).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::MonitorRemoved(monitor_name))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_monitor_added(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let monitor_name = data.to_string();
    hyprland_tx.send(HyprlandEvent::MonitorAdded { name: monitor_name })?;

    Ok(())
}

pub(crate) fn handle_monitor_added_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let event_data = format!("{event}>>{data}");
    let parts: Vec<&str> = data.split(',').collect();
    let [id, name, description] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data,
            field: "monitor_data",
            expected: "3 comma-separated values (id,name,description)",
            value: data.to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data,
        field: "monitor_id",
        expected: "integer",
        value: (*id).to_string(),
    })?;

    let monitor_name = (*name).to_string();
    hyprland_tx.send(HyprlandEvent::MonitorAddedV2 {
        id,
        name: monitor_name.clone(),
        description: (*description).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::MonitorCreated(monitor_name))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}
