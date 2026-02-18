use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;

use super::{AudioDropdown, messages::AudioDropdownCmd};

pub(super) fn spawn(sender: &ComponentSender<AudioDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(AudioDropdownCmd::ScaleChanged(scale.get().value()));
    });
}
