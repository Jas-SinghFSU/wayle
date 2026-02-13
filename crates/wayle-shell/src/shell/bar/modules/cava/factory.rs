use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{CavaInit, CavaModule};
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
        let init = CavaInit {
            settings: settings.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(CavaModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
