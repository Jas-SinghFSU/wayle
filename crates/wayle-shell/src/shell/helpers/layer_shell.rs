use std::sync::Arc;

use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::gtk;
use tracing::warn;
use wayle_config::ConfigService;

use super::monitors::current_monitors;

/// Clears all layer-shell edge anchors and zeroes margins.
pub(crate) fn reset_anchors(root: &gtk::Window) {
    root.set_anchor(Edge::Top, false);
    root.set_anchor(Edge::Bottom, false);
    root.set_anchor(Edge::Left, false);
    root.set_anchor(Edge::Right, false);

    root.set_margin(Edge::Top, 0);
    root.set_margin(Edge::Bottom, 0);
    root.set_margin(Edge::Left, 0);
    root.set_margin(Edge::Right, 0);
}

/// Switches between `Overlay` and `Top` layer for tearing mode compatibility.
pub(crate) fn apply_tearing_layer(root: &gtk::Window, config: &Arc<ConfigService>) {
    let tearing = config.config().general.tearing_mode.get();

    let layer = if tearing { Layer::Top } else { Layer::Overlay };

    root.set_layer(layer);
}

/// Resolves and applies a monitor by connector name, falling back to primary.
pub(crate) fn apply_monitor_by_connector(root: &gtk::Window, connector: &str) {
    let monitors = current_monitors();
    let mut primary = None;
    let mut matched = None;

    for (name, monitor) in monitors {
        if primary.is_none() {
            primary = Some(monitor.clone());
        }

        if name == connector {
            matched = Some(monitor);
            break;
        }
    }

    if matched.is_none() {
        warn!(
            connector,
            "configured monitor not found, falling back to primary"
        );
    }

    root.set_monitor(matched.or(primary).as_ref());
}

/// Assigns the first available monitor to the layer-shell surface.
pub(crate) fn apply_primary_monitor(root: &gtk::Window) {
    let monitors = current_monitors();

    let primary = monitors.into_iter().next().map(|(_, monitor)| monitor);

    root.set_monitor(primary.as_ref());
}
