use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::{watch, watchers::changes_stream};
use wayle_config::ConfigService;

use crate::shell::bar::{Bar, BarCmd};

pub(crate) fn spawn(sender: &ComponentSender<Bar>, config_service: &Arc<ConfigService>) {
    let config = config_service.config().clone();
    let location_prop = config.bar.location.clone();

    watch!(sender, [changes_stream(&config.bar.location)], |out| {
        let location = location_prop.get();
        let _ = out.send(BarCmd::LocationChanged(location));
    });
}
