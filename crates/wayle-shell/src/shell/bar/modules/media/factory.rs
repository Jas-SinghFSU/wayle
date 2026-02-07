use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{MediaInit, MediaModule};
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
        let media = require_service("media", "media", services.media.clone())?;

        let init = MediaInit {
            settings: settings.clone(),
            media,
            config: services.config.clone(),
        };
        let controller = dynamic_controller(MediaModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
