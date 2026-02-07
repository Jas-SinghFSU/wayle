use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{HyprsunsetInit, HyprsunsetModule};
use crate::shell::{
    bar::modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller, require_hyprland},
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        if !require_hyprland("hyprsunset") {
            return None;
        }

        let init = HyprsunsetInit {
            settings: settings.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(HyprsunsetModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
