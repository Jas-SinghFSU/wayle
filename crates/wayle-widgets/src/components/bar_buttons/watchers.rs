//! Async stream subscriptions for bar button.

use relm4::ComponentSender;
use wayle_config::{ConfigProperty, schemas::bar::IconPosition};

use super::{
    component::{BarButton, BarButtonCmd},
    types::BarButtonVariant,
};
use crate::watch;

pub(super) fn spawn_variant_watcher(
    variant: &ConfigProperty<BarButtonVariant>,
    sender: &ComponentSender<BarButton>,
) {
    let variant = variant.clone();
    watch!(sender, [variant.watch()], |out| {
        let _ = out.send(BarButtonCmd::VariantChanged(variant.get()));
    });
}

pub(super) fn spawn_icon_position_watcher(
    icon_position: &ConfigProperty<IconPosition>,
    sender: &ComponentSender<BarButton>,
) {
    let icon_position = icon_position.clone();
    watch!(sender, [icon_position.watch()], |out| {
        let _ = out.send(BarButtonCmd::IconPositionChanged(icon_position.get()));
    });
}
