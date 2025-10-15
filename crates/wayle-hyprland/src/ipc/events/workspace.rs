use tokio::sync::broadcast::Sender;

use super::types::ServiceNotification;
use crate::{Error, HyprlandEvent, Result};

pub(crate) fn handle_workspace(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::Workspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_workspace_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((id, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated id,name".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::WorkspaceV2 {
        id,
        name: name.to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceFocused(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_create_workspace(
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::CreateWorkspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_create_workspace_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((id, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated id,name".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::CreateWorkspaceV2 {
        id,
        name: name.to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceCreated(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_destroy_workspace(
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::DestroyWorkspace {
        name: data.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_destroy_workspace_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((id, name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated id,name".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::DestroyWorkspaceV2 {
        id,
        name: name.to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceRemoved(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_move_workspace(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((name, monitor)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated name,monitor".to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::MoveWorkspace {
        name: name.to_string(),
        monitor: monitor.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_move_workspace_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let parts: Vec<&str> = data.split(',').collect();
    let [id, name, monitor] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected 3 comma-separated values (id,name,monitor)".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::MoveWorkspaceV2 {
        id,
        name: (*name).to_string(),
        monitor: (*monitor).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceMoved(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_rename_workspace(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((id, new_name)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated id,new_name".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::RenameWorkspace {
        id,
        new_name: new_name.to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceUpdated(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}

pub(crate) fn handle_active_special(
    event: &str,
    data: &str,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((workspace, monitor)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated workspace,monitor".to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveSpecial {
        workspace: workspace.to_string(),
        monitor: monitor.to_string(),
    })?;

    Ok(())
}

pub(crate) fn handle_active_special_v2(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let parts: Vec<&str> = data.split(',').collect();
    let [id, workspace, monitor] = parts.as_slice() else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected 3 comma-separated values (id,workspace,monitor)".to_string(),
        });
    };
    let id = id.parse().map_err(|_| Error::EventParseError {
        event_data: format!("{event}>>{data}"),
        reason: format!("invalid workspace ID: {id}"),
    })?;

    hyprland_tx.send(HyprlandEvent::ActiveSpecialV2 {
        id,
        workspace: (*workspace).to_string(),
        monitor: (*monitor).to_string(),
    })?;

    internal_tx
        .send(ServiceNotification::WorkspaceUpdated(id))
        .map_err(|e| Error::InternalEventTransmitError(e.to_string()))?;

    Ok(())
}
