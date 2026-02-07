use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{IdleInhibitInit, IdleInhibitModule};
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
        let init = IdleInhibitInit {
            settings: settings.clone(),
            idle_inhibit: services.idle_inhibit.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(IdleInhibitModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
