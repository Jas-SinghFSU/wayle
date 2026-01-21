use relm4::{
    ComponentSender,
    gtk::{gdk, prelude::*},
};
use tokio::sync::mpsc;
use tracing::{debug, warn};
use wayle_common::{SubscribeChanges, services};
use wayle_config::{ConfigService, schemas::bar::BarLayout};

use crate::shell::bar::{Bar, BarCmd};

pub(crate) fn spawn(sender: &ComponentSender<Bar>, monitor: &gdk::Monitor) {
    let config = services::get::<ConfigService>().config().clone();
    let connector = monitor
        .connector()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let (tx, mut rx) = mpsc::unbounded_channel();
    config.bar.layout.subscribe_changes(tx);

    sender.command(move |out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        let load_layout = || {
            let layouts = config.bar.layout.get();
            debug!(connector = %connector, layout_count = layouts.len(), "Loaded bar layouts");
            resolve_layout(&layouts, &connector)
        };

        if let Some(layout) = load_layout() {
            debug!(connector = %connector, monitor = %layout.monitor, "Sending initial layout");
            let _ = out.send(BarCmd::LayoutLoaded(layout));
        } else {
            warn!(connector = %connector, "No bar layout found");
        }

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                Some(()) = rx.recv() => {
                    if let Some(layout) = load_layout() {
                        let _ = out.send(BarCmd::LayoutLoaded(layout));
                    }
                }
            }
        }
    });
}

fn resolve_layout(layouts: &[BarLayout], connector: &str) -> Option<BarLayout> {
    if let Some(layout) = layouts.iter().find(|l| l.monitor == connector) {
        return Some(resolve_extends(layout, layouts));
    }

    if let Some(layout) = layouts.iter().find(|l| l.monitor == "*") {
        return Some(resolve_extends(layout, layouts));
    }

    None
}

fn resolve_extends(layout: &BarLayout, all_layouts: &[BarLayout]) -> BarLayout {
    let mut resolved = layout.clone();

    if let Some(ref extends_name) = layout.extends
        && let Some(parent) = all_layouts.iter().find(|l| l.monitor == *extends_name) {
            let parent_resolved = resolve_extends(parent, all_layouts);

            if resolved.left.is_empty() {
                resolved.left = parent_resolved.left;
            }
            if resolved.center.is_empty() {
                resolved.center = parent_resolved.center;
            }
            if resolved.right.is_empty() {
                resolved.right = parent_resolved.right;
            }
        }

    resolved
}
