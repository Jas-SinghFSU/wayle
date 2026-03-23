use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::ConfigService;
use wayle_widgets::watch;

use super::{BatteryDropdown, messages::BatteryDropdownCmd};

pub(super) fn spawn(sender: &ComponentSender<BatteryDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(BatteryDropdownCmd::ScaleChanged(scale.get().value()));
    });
}
