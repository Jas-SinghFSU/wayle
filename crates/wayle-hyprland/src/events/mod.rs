use std::env;

use layer::{handle_close_layer, handle_open_layer};
use monitor::{
    handle_focused_mon, handle_focused_mon_v2, handle_monitor_added, handle_monitor_added_v2,
    handle_monitor_removed, handle_monitor_removed_v2,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
    sync::broadcast::Sender,
};
use tracing::warn;
use types::{HyprlandEvent, ServiceNotification};
use window::{
    handle_active_window, handle_active_window_v2, handle_change_floating_mode,
    handle_close_window, handle_minimized, handle_move_into_group, handle_move_out_of_group,
    handle_move_window, handle_move_window_v2, handle_open_window, handle_pin, handle_toggle_group,
    handle_urgent, handle_window_title, handle_window_title_v2,
};
use workspace::{
    handle_active_special, handle_active_special_v2, handle_create_workspace,
    handle_create_workspace_v2, handle_destroy_workspace, handle_destroy_workspace_v2,
    handle_move_workspace, handle_move_workspace_v2, handle_rename_workspace, handle_workspace,
    handle_workspace_v2,
};

use crate::{Address, Error, Result, ScreencastOwner};

pub mod layer;
pub mod monitor;
pub mod types;
pub mod window;
pub mod workspace;

pub(crate) async fn subscribe(
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::HyprlandNotRunning)?;
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map_err(|_| Error::InvalidInstanceSignature("XDG_RUNTIME_DIR not set".to_string()))?;

    let socket_name = format!("{runtime_dir}/hypr/{his}/.socket2.sock");
    let event_stream =
        UnixStream::connect(&socket_name)
            .await
            .map_err(|e| Error::IpcConnectionFailed {
                socket_type: "event",
                reason: e.to_string(),
            })?;

    tokio::spawn(async move {
        let reader = BufReader::new(event_stream);
        let mut lines = reader.lines();

        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let Some((event, data)) = line.split_once(">>") else {
                        warn!("Failed to parse Hyprland event: missing '>>' separator");
                        warn!("Data: {line}");
                        continue;
                    };

                    if let Err(e) =
                        handle_event(event, data, internal_tx.clone(), hyprland_tx.clone()).await
                    {
                        warn!("Failed to handle event {event}: {e}");
                    }
                }
                Ok(None) => {
                    warn!("Hyprland event stream closed");
                    break;
                }
                Err(e) => {
                    warn!("Error reading event stream: {e}");
                    break;
                }
            }
        }
    });

    Ok(())
}

async fn handle_event(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    match event {
        "workspace" => handle_workspace(event, data, internal_tx, hyprland_tx),
        "workspacev2" => handle_workspace_v2(event, data, internal_tx, hyprland_tx),
        "focusedmon" => handle_focused_mon(event, data, internal_tx, hyprland_tx),
        "focusedmonv2" => handle_focused_mon_v2(event, data, internal_tx, hyprland_tx),
        "activewindow" => handle_active_window(event, data, internal_tx, hyprland_tx),
        "activewindowv2" => handle_active_window_v2(event, data, internal_tx, hyprland_tx),
        "fullscreen" => handle_fullscreen(event, data, internal_tx, hyprland_tx),
        "monitorremoved" => handle_monitor_removed(event, data, internal_tx, hyprland_tx),
        "monitorremovedv2" => handle_monitor_removed_v2(event, data, internal_tx, hyprland_tx),
        "monitoradded" => handle_monitor_added(event, data, internal_tx, hyprland_tx),
        "monitoraddedv2" => handle_monitor_added_v2(event, data, internal_tx, hyprland_tx),
        "createworkspace" => handle_create_workspace(event, data, internal_tx, hyprland_tx),
        "createworkspacev2" => handle_create_workspace_v2(event, data, internal_tx, hyprland_tx),
        "destroyworkspace" => handle_destroy_workspace(event, data, internal_tx, hyprland_tx),
        "destroyworkspacev2" => handle_destroy_workspace_v2(event, data, internal_tx, hyprland_tx),
        "moveworkspace" => handle_move_workspace(event, data, internal_tx, hyprland_tx),
        "moveworkspacev2" => handle_move_workspace_v2(event, data, internal_tx, hyprland_tx),
        "renameworkspace" => handle_rename_workspace(event, data, internal_tx, hyprland_tx),
        "activespecial" => handle_active_special(event, data, internal_tx, hyprland_tx),
        "activespecialv2" => handle_active_special_v2(event, data, internal_tx, hyprland_tx),
        "activelayout" => handle_active_layout(event, data, internal_tx, hyprland_tx),
        "openwindow" => handle_open_window(event, data, internal_tx, hyprland_tx),
        "closewindow" => handle_close_window(event, data, internal_tx, hyprland_tx),
        "movewindow" => handle_move_window(event, data, internal_tx, hyprland_tx),
        "movewindowv2" => handle_move_window_v2(event, data, internal_tx, hyprland_tx),
        "openlayer" => handle_open_layer(event, data, internal_tx, hyprland_tx),
        "closelayer" => handle_close_layer(event, data, internal_tx, hyprland_tx),
        "submap" => handle_submap(event, data, internal_tx, hyprland_tx),
        "changefloatingmode" => handle_change_floating_mode(event, data, internal_tx, hyprland_tx),
        "urgent" => handle_urgent(event, data, internal_tx, hyprland_tx),
        "screencast" => handle_screencast(event, data, internal_tx, hyprland_tx),
        "windowtitle" => handle_window_title(event, data, internal_tx, hyprland_tx),
        "windowtitlev2" => handle_window_title_v2(event, data, internal_tx, hyprland_tx),
        "togglegroup" => handle_toggle_group(event, data, internal_tx, hyprland_tx),
        "moveintogroup" => handle_move_into_group(event, data, internal_tx, hyprland_tx),
        "moveoutofgroup" => handle_move_out_of_group(event, data, internal_tx, hyprland_tx),
        "ignoregrouplock" => handle_ignore_group_lock(event, data, internal_tx, hyprland_tx),
        "lockgroups" => handle_lock_groups(event, data, internal_tx, hyprland_tx),
        "configreloaded" => handle_config_reloaded(event, data, internal_tx, hyprland_tx),
        "pin" => handle_pin(event, data, internal_tx, hyprland_tx),
        "minimized" => handle_minimized(event, data, internal_tx, hyprland_tx),
        "bell" => handle_bell(event, data, internal_tx, hyprland_tx),
        _ => {
            warn!("Unknown Hyprland event: {event}");
            Ok(())
        }
    }
}

fn handle_fullscreen(
    event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let fullscreen = match data {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid fullscreen value: {data}"),
            });
        }
    };
    hyprland_tx.send(HyprlandEvent::Fullscreen { fullscreen })?;

    Ok(())
}

fn handle_active_layout(
    event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((keyboard, layout)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated keyboard,layout".to_string(),
        });
    };

    hyprland_tx.send(HyprlandEvent::ActiveLayout {
        keyboard: keyboard.to_string(),
        layout: layout.to_string(),
    })?;

    Ok(())
}

fn handle_submap(
    _event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::Submap {
        name: data.to_string(),
    })?;

    Ok(())
}

fn handle_screencast(
    event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let Some((state, owner)) = data.split_once(',') else {
        return Err(Error::EventParseError {
            event_data: format!("{event}>>{data}"),
            reason: "expected comma-separated state,owner".to_string(),
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

    let owner = ScreencastOwner::try_from(owner)?;

    hyprland_tx.send(HyprlandEvent::Screencast { state, owner })?;

    Ok(())
}

fn handle_ignore_group_lock(
    event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let ignore = match data {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid ignore value: {data}"),
            });
        }
    };

    hyprland_tx.send(HyprlandEvent::IgnoreGroupLock { ignore })?;

    Ok(())
}

fn handle_lock_groups(
    event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let locked = match data {
        "0" => false,
        "1" => true,
        _ => {
            return Err(Error::EventParseError {
                event_data: format!("{event}>>{data}"),
                reason: format!("invalid locked value: {data}"),
            });
        }
    };

    hyprland_tx.send(HyprlandEvent::LockGroups { locked })?;

    Ok(())
}

fn handle_config_reloaded(
    _event: &str,
    _data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::ConfigReloaded)?;

    Ok(())
}

fn handle_bell(
    _event: &str,
    data: &str,
    _internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    let address = if data.is_empty() {
        None
    } else {
        Some(Address::new(data.to_string()))
    };

    hyprland_tx.send(HyprlandEvent::Bell { address })?;

    Ok(())
}
