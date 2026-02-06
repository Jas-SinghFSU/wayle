use std::sync::Arc;

use futures::StreamExt;
use relm4::ComponentSender;
use tracing::warn;
use wayle_common::watch;
use wayle_config::schemas::modules::KeybindModeConfig;
use wayle_hyprland::{HyprlandEvent, HyprlandService};

use super::HyprlandKeybindMode;
use crate::shell::bar::modules::keybind_mode::messages::KeybindModeCmd;

pub(super) fn spawn_watchers(
    sender: &ComponentSender<HyprlandKeybindMode>,
    config: &KeybindModeConfig,
    hyprland: &Option<Arc<HyprlandService>>,
) {
    spawn_mode_watcher(sender, config, hyprland);
    spawn_config_watchers(sender, config);
}

fn spawn_mode_watcher(
    sender: &ComponentSender<HyprlandKeybindMode>,
    config: &KeybindModeConfig,
    hyprland: &Option<Arc<HyprlandService>>,
) {
    let Some(hyprland) = hyprland.clone() else {
        warn!(
            service = "HyprlandService",
            module = "keybind-mode",
            "unavailable, skipping watcher"
        );
        return;
    };

    let format = config.format.clone();
    sender.command(move |out, shutdown| {
        watch_mode_events(hyprland.clone(), format.clone(), out, shutdown)
    });
}

async fn watch_mode_events(
    hyprland: Arc<HyprlandService>,
    format: wayle_common::ConfigProperty<String>,
    out: relm4::Sender<KeybindModeCmd>,
    shutdown: relm4::ShutdownReceiver,
) {
    if let Ok(name) = hyprland.submap().await {
        let _ = out.send(KeybindModeCmd::ModeChanged {
            name,
            format: format.get(),
        });
    }

    let mut events = hyprland.events();
    let shutdown_fut = shutdown.wait();
    tokio::pin!(shutdown_fut);

    loop {
        tokio::select! {
            () = &mut shutdown_fut => return,
            event = events.next() => {
                let Some(HyprlandEvent::Submap { name }) = event else {
                    continue;
                };
                let _ = out.send(KeybindModeCmd::ModeChanged {
                    name,
                    format: format.get(),
                });
            }
        }
    }
}

fn spawn_config_watchers(
    sender: &ComponentSender<HyprlandKeybindMode>,
    config: &KeybindModeConfig,
) {
    let format = config.format.clone();
    watch!(sender, [format.watch()], |out| {
        let _ = out.send(KeybindModeCmd::FormatChanged);
    });

    let auto_hide = config.auto_hide.clone();
    watch!(sender, [auto_hide.watch()], |out| {
        let _ = out.send(KeybindModeCmd::AutoHideChanged);
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(KeybindModeCmd::UpdateIcon(icon_name.get().clone()));
    });
}
