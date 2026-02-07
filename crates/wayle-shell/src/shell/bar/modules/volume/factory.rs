use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{VolumeInit, VolumeModule};
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
        let audio = require_service("volume", "audio", services.audio.clone())?;

        let init = VolumeInit {
            settings: settings.clone(),
            audio,
            config: services.config.clone(),
        };
        let controller = dynamic_controller(VolumeModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
