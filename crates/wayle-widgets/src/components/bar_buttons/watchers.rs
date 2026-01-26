//! Async stream subscriptions for bar button.

use relm4::ComponentSender;
use wayle_common::{ConfigProperty, watch};

use super::{
    component::{BarButton, BarButtonCmd},
    types::BarButtonVariant,
};

pub(super) fn spawn_variant_watcher(
    variant: &ConfigProperty<BarButtonVariant>,
    sender: &ComponentSender<BarButton>,
) {
    let variant = variant.clone();
    watch!(sender, [variant.watch()], |out| {
        let _ = out.send(BarButtonCmd::VariantChanged(variant.get()));
    });
}
