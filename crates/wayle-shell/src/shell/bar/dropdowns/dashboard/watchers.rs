use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;

use super::{DashboardDropdown, messages::DashboardDropdownCmd};

pub(super) fn spawn(sender: &ComponentSender<DashboardDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();
    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(DashboardDropdownCmd::ScaleChanged(scale.get().value()));
    });
}
