use tokio::sync::broadcast::Sender;
use tracing::warn;

use super::{
    layer::{handle_close_layer, handle_open_layer},
    monitor::{
        handle_focused_mon, handle_focused_mon_v2, handle_monitor_added, handle_monitor_added_v2,
        handle_monitor_removed, handle_monitor_removed_v2,
    },
    types::{HyprlandEvent, ServiceNotification},
    window::{
        handle_active_window, handle_active_window_v2, handle_change_floating_mode,
        handle_close_window, handle_minimized, handle_move_into_group, handle_move_out_of_group,
        handle_move_window, handle_move_window_v2, handle_open_window, handle_pin,
        handle_toggle_group, handle_urgent, handle_window_title, handle_window_title_v2,
    },
    workspace::{
        handle_active_special, handle_active_special_v2, handle_create_workspace,
        handle_create_workspace_v2, handle_destroy_workspace, handle_destroy_workspace_v2,
        handle_move_workspace, handle_move_workspace_v2, handle_rename_workspace, handle_workspace,
        handle_workspace_v2,
    },
};
use crate::{Address, Error, Result, ScreencastOwner};

pub(crate) async fn dispatch(
    event: &str,
    data: &str,
    internal_tx: Sender<ServiceNotification>,
    hyprland_tx: Sender<HyprlandEvent>,
) -> Result<()> {
    match event {
        "workspace" => handle_workspace(data, hyprland_tx),
        "workspacev2" => handle_workspace_v2(event, data, internal_tx, hyprland_tx),
        "focusedmon" => handle_focused_mon(event, data, hyprland_tx),
        "focusedmonv2" => handle_focused_mon_v2(event, data, internal_tx, hyprland_tx),
        "activewindow" => handle_active_window(event, data, hyprland_tx),
        "activewindowv2" => handle_active_window_v2(data, internal_tx, hyprland_tx),
        "fullscreen" => handle_fullscreen(event, data, hyprland_tx),
        "monitorremoved" => handle_monitor_removed(data, hyprland_tx),
        "monitorremovedv2" => handle_monitor_removed_v2(event, data, internal_tx, hyprland_tx),
        "monitoradded" => handle_monitor_added(data, hyprland_tx),
        "monitoraddedv2" => handle_monitor_added_v2(event, data, internal_tx, hyprland_tx),
        "createworkspace" => handle_create_workspace(data, hyprland_tx),
        "createworkspacev2" => handle_create_workspace_v2(event, data, internal_tx, hyprland_tx),
        "destroyworkspace" => handle_destroy_workspace(data, hyprland_tx),
        "destroyworkspacev2" => handle_destroy_workspace_v2(event, data, internal_tx, hyprland_tx),
        "moveworkspace" => handle_move_workspace(event, data, hyprland_tx),
        "moveworkspacev2" => handle_move_workspace_v2(event, data, internal_tx, hyprland_tx),
        "renameworkspace" => handle_rename_workspace(event, data, internal_tx, hyprland_tx),
        "activespecial" => handle_active_special(event, data, hyprland_tx),
        "activespecialv2" => handle_active_special_v2(event, data, internal_tx, hyprland_tx),
        "activelayout" => handle_active_layout(event, data, hyprland_tx),
        "openwindow" => handle_open_window(event, data, internal_tx, hyprland_tx),
        "closewindow" => handle_close_window(data, internal_tx, hyprland_tx),
        "movewindow" => handle_move_window(event, data, hyprland_tx),
        "movewindowv2" => handle_move_window_v2(event, data, internal_tx, hyprland_tx),
        "openlayer" => handle_open_layer(data, internal_tx, hyprland_tx),
        "closelayer" => handle_close_layer(data, internal_tx, hyprland_tx),
        "submap" => handle_submap(data, hyprland_tx),
        "changefloatingmode" => handle_change_floating_mode(event, data, internal_tx, hyprland_tx),
        "urgent" => handle_urgent(data, hyprland_tx),
        "screencast" => handle_screencast(event, data, hyprland_tx),
        "windowtitle" => handle_window_title(data, internal_tx, hyprland_tx),
        "windowtitlev2" => handle_window_title_v2(event, data, internal_tx, hyprland_tx),
        "togglegroup" => handle_toggle_group(event, data, internal_tx, hyprland_tx),
        "moveintogroup" => handle_move_into_group(data, internal_tx, hyprland_tx),
        "moveoutofgroup" => handle_move_out_of_group(data, internal_tx, hyprland_tx),
        "ignoregrouplock" => handle_ignore_group_lock(event, data, hyprland_tx),
        "lockgroups" => handle_lock_groups(event, data, hyprland_tx),
        "configreloaded" => handle_config_reloaded(hyprland_tx),
        "pin" => handle_pin(event, data, internal_tx, hyprland_tx),
        "minimized" => handle_minimized(event, data, hyprland_tx),
        "bell" => handle_bell(data, hyprland_tx),
        _ => {
            warn!("Unknown Hyprland event: {event}");
            Ok(())
        }
    }
}

fn handle_fullscreen(event: &str, data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
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

fn handle_active_layout(event: &str, data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
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

fn handle_submap(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::Submap {
        name: data.to_string(),
    })?;

    Ok(())
}

fn handle_screencast(event: &str, data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
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

fn handle_lock_groups(event: &str, data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
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

fn handle_config_reloaded(hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    hyprland_tx.send(HyprlandEvent::ConfigReloaded)?;

    Ok(())
}

fn handle_bell(data: &str, hyprland_tx: Sender<HyprlandEvent>) -> Result<()> {
    let address = if data.is_empty() {
        None
    } else {
        Some(Address::new(data.to_string()))
    };

    hyprland_tx.send(HyprlandEvent::Bell { address })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;

    use super::*;

    #[test]
    fn handle_fullscreen_parses_false_correctly() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_fullscreen("fullscreen", "0", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_fullscreen_parses_true_correctly() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_fullscreen("fullscreen", "1", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_fullscreen_returns_error_for_invalid_value() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_fullscreen("fullscreen", "2", tx);

        assert!(result.is_err());
        if let Err(Error::EventParseError { event_data, reason }) = result {
            assert!(event_data.contains("fullscreen"));
            assert!(event_data.contains("2"));
            assert!(reason.contains("invalid fullscreen value"));
        } else {
            panic!("Expected EventParseError");
        }
    }

    #[test]
    fn handle_active_layout_parses_valid_data() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_active_layout("activelayout", "keyboard1,us", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_active_layout_returns_error_without_comma() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_active_layout("activelayout", "no_comma_here", tx);

        assert!(result.is_err());
        if let Err(Error::EventParseError { reason, .. }) = result {
            assert!(reason.contains("comma-separated"));
        } else {
            panic!("Expected EventParseError");
        }
    }

    #[test]
    fn handle_screencast_parses_valid_state_and_owner() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_screencast("screencast", "1,1", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_screencast_returns_error_for_invalid_state() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_screencast("screencast", "5,0", tx);

        assert!(result.is_err());
        if let Err(Error::EventParseError { reason, .. }) = result {
            assert!(reason.contains("invalid state value"));
        } else {
            panic!("Expected EventParseError");
        }
    }

    #[test]
    fn handle_screencast_returns_error_without_comma() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_screencast("screencast", "no_comma", tx);

        assert!(result.is_err());
        if let Err(Error::EventParseError { reason, .. }) = result {
            assert!(reason.contains("comma-separated"));
        } else {
            panic!("Expected EventParseError");
        }
    }

    #[test]
    fn handle_screencast_returns_error_for_invalid_owner() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_screencast("screencast", "1,99", tx);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::InvalidEnumValue { .. }
        ));
    }

    #[test]
    fn handle_bell_creates_some_address_for_non_empty_data() {
        let (tx, mut rx) = broadcast::channel(10);

        let result = handle_bell("0xdeadbeef", tx);

        assert!(result.is_ok());
        let event = rx.try_recv().unwrap();
        if let HyprlandEvent::Bell { address } = event {
            assert!(address.is_some());
            assert_eq!(address.unwrap().as_str(), "deadbeef");
        } else {
            panic!("Expected Bell event");
        }
    }

    #[test]
    fn handle_bell_creates_none_address_for_empty_data() {
        let (tx, mut rx) = broadcast::channel(10);

        let result = handle_bell("", tx);

        assert!(result.is_ok());
        let event = rx.try_recv().unwrap();
        if let HyprlandEvent::Bell { address } = event {
            assert!(address.is_none());
        } else {
            panic!("Expected Bell event");
        }
    }

    #[test]
    fn handle_ignore_group_lock_parses_true() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_ignore_group_lock("ignoregrouplock", "1", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_ignore_group_lock_parses_false() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_ignore_group_lock("ignoregrouplock", "0", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_ignore_group_lock_returns_error_for_invalid() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_ignore_group_lock("ignoregrouplock", "invalid", tx);

        assert!(result.is_err());
    }

    #[test]
    fn handle_lock_groups_parses_true() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_lock_groups("lockgroups", "1", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_lock_groups_parses_false() {
        let (tx, _rx) = broadcast::channel(10);

        let result = handle_lock_groups("lockgroups", "0", tx);

        assert!(result.is_ok() || matches!(result, Err(Error::HyprlandEventTransmitError(_))));
    }

    #[test]
    fn handle_lock_groups_returns_error_for_invalid() {
        let (tx, _) = broadcast::channel(10);

        let result = handle_lock_groups("lockgroups", "99", tx);

        assert!(result.is_err());
    }

    #[test]
    fn handle_config_reloaded_sends_event() {
        let (tx, mut rx) = broadcast::channel(10);

        let result = handle_config_reloaded(tx);

        assert!(result.is_ok());
        let event = rx.try_recv().unwrap();
        assert!(matches!(event, HyprlandEvent::ConfigReloaded));
    }

    #[test]
    fn handle_submap_sends_event_with_name() {
        let (tx, mut rx) = broadcast::channel(10);

        let result = handle_submap("resize", tx);

        assert!(result.is_ok());
        let event = rx.try_recv().unwrap();
        if let HyprlandEvent::Submap { name } = event {
            assert_eq!(name, "resize");
        } else {
            panic!("Expected Submap event");
        }
    }
}
