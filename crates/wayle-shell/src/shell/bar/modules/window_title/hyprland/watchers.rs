use std::sync::Arc;

use futures::StreamExt;
use relm4::ComponentSender;
use tracing::warn;
use wayle_common::{ConfigProperty, services, watch};
use wayle_config::schemas::modules::WindowTitleConfig;
use wayle_hyprland::{HyprlandEvent, HyprlandService};

use super::HyprlandWindowTitle;
use crate::shell::bar::modules::window_title::WindowTitleCmd;

pub(super) fn spawn_watchers(
    sender: &ComponentSender<HyprlandWindowTitle>,
    config: &WindowTitleConfig,
) {
    spawn_window_watcher(sender, config);
    spawn_config_watchers(sender, config);
}

fn spawn_window_watcher(sender: &ComponentSender<HyprlandWindowTitle>, config: &WindowTitleConfig) {
    let Some(hyprland) = services::try_get::<HyprlandService>() else {
        warn!(
            service = "HyprlandService",
            module = "window-title",
            "unavailable, skipping watcher"
        );
        return;
    };

    let format = config.format.clone();
    sender.command(move |out, shutdown| {
        watch_window_events(hyprland.clone(), format.clone(), out, shutdown)
    });
}

async fn watch_window_events(
    hyprland: Arc<HyprlandService>,
    format: ConfigProperty<String>,
    out: relm4::Sender<WindowTitleCmd>,
    shutdown: relm4::ShutdownReceiver,
) {
    let mut events = hyprland.events();
    let shutdown_fut = shutdown.wait();
    tokio::pin!(shutdown_fut);

    loop {
        tokio::select! {
            () = &mut shutdown_fut => return,
            event = events.next() => {
                let Some(event) = event else { continue };
                match event {
                    HyprlandEvent::ActiveWindow { class, title } => {
                        let _ = out.send(WindowTitleCmd::WindowChanged {
                            title,
                            class,
                            format: format.get(),
                        });
                    }
                    HyprlandEvent::WindowTitleV2 { address, title } => {
                        let Some(active) = hyprland.active_window().await else {
                            continue;
                        };
                        if active.address.get() != address {
                            continue;
                        }
                        let _ = out.send(WindowTitleCmd::WindowChanged {
                            title,
                            class: active.class.get(),
                            format: format.get(),
                        });
                    }
                    _ => continue,
                }
            }
        }
    }
}

fn spawn_config_watchers(
    sender: &ComponentSender<HyprlandWindowTitle>,
    config: &WindowTitleConfig,
) {
    let format = config.format.clone();
    watch!(sender, [format.watch()], |out| {
        let _ = out.send(WindowTitleCmd::FormatChanged);
    });

    let icon_name = config.icon_name.clone();
    let icon_mappings = config.icon_mappings.clone();
    watch!(sender, [icon_name.watch(), icon_mappings.watch()], |out| {
        let _ = out.send(WindowTitleCmd::IconConfigChanged);
    });
}
