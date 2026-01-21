//! Layer-shell positioning and GTK widget orientation for bars.

use gtk::prelude::*;
use gtk4_layer_shell::{Edge, LayerShell};
use relm4::{gtk, gtk::gdk};
use wayle_config::schemas::bar::Location;
use wayle_widgets::styling::InlineStyling;

use super::Bar;

impl Bar {
    pub(super) fn apply_anchors(window: &gtk::Window, location: Location) {
        let (anchor_edge, stretch_edges) = match location {
            Location::Top => (Edge::Top, [Edge::Left, Edge::Right]),
            Location::Bottom => (Edge::Bottom, [Edge::Left, Edge::Right]),
            Location::Left => (Edge::Left, [Edge::Top, Edge::Bottom]),
            Location::Right => (Edge::Right, [Edge::Top, Edge::Bottom]),
        };

        window.set_anchor(Edge::Top, false);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Right, false);

        window.set_anchor(anchor_edge, true);
        for edge in stretch_edges {
            window.set_anchor(edge, true);
        }
    }

    pub(super) fn apply_css_classes(
        window: &gtk::Window,
        monitor: &gdk::Monitor,
        location: Location,
        is_floating: bool,
    ) {
        if let Some(connector) = monitor.connector() {
            window.add_css_class(&connector);
        }

        window.add_css_class(location.css_class());

        if is_floating {
            window.add_css_class("floating");
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_orientations(
        center_box: &gtk::CenterBox,
        left_box: &gtk::Box,
        middle_box: &gtk::Box,
        right_box: &gtk::Box,
        left_factory: &gtk::Box,
        center_factory: &gtk::Box,
        right_factory: &gtk::Box,
        is_vertical: bool,
    ) {
        let orientation = if is_vertical {
            gtk::Orientation::Vertical
        } else {
            gtk::Orientation::Horizontal
        };

        center_box.set_orientation(orientation);
        left_box.set_orientation(orientation);
        middle_box.set_orientation(orientation);
        right_box.set_orientation(orientation);

        left_factory.set_orientation(orientation);
        center_factory.set_orientation(orientation);
        right_factory.set_orientation(orientation);

        left_box.set_vexpand(false);
        middle_box.set_vexpand(false);
        right_box.set_vexpand(false);
        left_box.set_hexpand(false);
        middle_box.set_hexpand(false);
        right_box.set_hexpand(false);
    }

    pub(super) fn apply_location_change(&mut self, root: &gtk::Window, location: Location) {
        if self.location == location {
            return;
        }

        root.remove_css_class(self.location.css_class());
        root.add_css_class(location.css_class());

        Self::apply_anchors(root, location);

        let is_vert = location.is_vertical();
        self.settings.is_vertical.set(is_vert);
        self.location = location;

        if let Some(center_box) = root
            .child()
            .and_then(|c| c.downcast::<gtk::CenterBox>().ok())
            && let (Some(left_box), Some(middle_box), Some(right_box)) = (
                center_box
                    .start_widget()
                    .and_then(|w| w.downcast::<gtk::Box>().ok()),
                center_box
                    .center_widget()
                    .and_then(|w| w.downcast::<gtk::Box>().ok()),
                center_box
                    .end_widget()
                    .and_then(|w| w.downcast::<gtk::Box>().ok()),
            )
        {
            Self::apply_orientations(
                &center_box,
                &left_box,
                &middle_box,
                &right_box,
                self.left.widget(),
                self.center.widget(),
                self.right.widget(),
                is_vert,
            );
        }

        self.reload_css();
    }
}
