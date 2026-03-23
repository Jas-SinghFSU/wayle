//! Async stream subscriptions for bar container.

use relm4::ComponentSender;
use wayle_config::ConfigProperty;

use super::component::{BarContainer, BarContainerCmd};
use crate::watch;

pub(super) fn spawn_orientation_watcher(
    is_vertical: &ConfigProperty<bool>,
    sender: &ComponentSender<BarContainer>,
) {
    let is_vertical = is_vertical.clone();
    watch!(sender, [is_vertical.watch()], |out| {
        let _ = out.send(BarContainerCmd::OrientationChanged(is_vertical.get()));
    });
}
