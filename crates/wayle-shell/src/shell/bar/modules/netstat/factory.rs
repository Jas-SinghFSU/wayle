use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{NetstatInit, NetstatModule};
use crate::shell::{
    bar::modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller},
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let init = NetstatInit {
            settings: settings.clone(),
            sysinfo: services.sysinfo.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(NetstatModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
