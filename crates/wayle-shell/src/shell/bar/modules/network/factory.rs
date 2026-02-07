use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{NetworkInit, NetworkModule};
use crate::shell::{
    bar::modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller, require_service},
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let network = require_service("network", "network", services.network.clone())?;

        let init = NetworkInit {
            settings: settings.clone(),
            network,
            config: services.config.clone(),
        };
        let controller = dynamic_controller(NetworkModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
