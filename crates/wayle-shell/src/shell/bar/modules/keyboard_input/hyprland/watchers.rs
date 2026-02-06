use std::sync::Arc;

use futures::StreamExt;
use relm4::ComponentSender;
use tracing::warn;
use wayle_common::{ConfigProperty, watch};
use wayle_config::schemas::modules::KeyboardInputConfig;
use wayle_hyprland::{HyprlandEvent, HyprlandService};

use super::HyprlandKeyboardInput;
use crate::shell::bar::modules::keyboard_input::KeyboardInputCmd;

pub(super) fn spawn_watchers(
    sender: &ComponentSender<HyprlandKeyboardInput>,
    config: &KeyboardInputConfig,
    hyprland: &Option<Arc<HyprlandService>>,
) {
    spawn_layout_watcher(sender, config, hyprland);
    spawn_config_watchers(sender, config);
}

fn spawn_layout_watcher(
    sender: &ComponentSender<HyprlandKeyboardInput>,
    config: &KeyboardInputConfig,
    hyprland: &Option<Arc<HyprlandService>>,
) {
    let Some(hyprland) = hyprland.clone() else {
        warn!(
            service = "HyprlandService",
            module = "keyboard-input",
            "unavailable, skipping watcher"
        );
        return;
    };

    let format = config.format.clone();
    sender.command(move |out, shutdown| {
        watch_layout_events(hyprland.clone(), format.clone(), out, shutdown)
    });
}

async fn watch_layout_events(
    hyprland: Arc<HyprlandService>,
    format: ConfigProperty<String>,
    out: relm4::Sender<KeyboardInputCmd>,
    shutdown: relm4::ShutdownReceiver,
) {
    let mut events = hyprland.events();
    let shutdown_fut = shutdown.wait();
    tokio::pin!(shutdown_fut);

    loop {
        tokio::select! {
            () = &mut shutdown_fut => return,
            event = events.next() => {
                let Some(HyprlandEvent::ActiveLayout { layout, .. }) = event else {
                    continue;
                };
                let _ = out.send(KeyboardInputCmd::LayoutChanged {
                    layout,
                    format: format.get(),
                });
            }
        }
    }
}

fn spawn_config_watchers(
    sender: &ComponentSender<HyprlandKeyboardInput>,
    config: &KeyboardInputConfig,
) {
    let format = config.format.clone();
    watch!(sender, [format.watch()], |out| {
        let _ = out.send(KeyboardInputCmd::FormatChanged);
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(KeyboardInputCmd::UpdateIcon(icon_name.get().clone()));
    });
}
