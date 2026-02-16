use std::rc::Rc;

use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{MicrophoneInit, MicrophoneModule};
use crate::shell::{
    bar::{
        dropdowns::DropdownRegistry,
        modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller, require_service},
    },
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        dropdowns: &Rc<DropdownRegistry>,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let audio = require_service("microphone", "audio", services.audio.clone())?;

        let init = MicrophoneInit {
            settings: settings.clone(),
            audio,
            config: services.config.clone(),
            dropdowns: dropdowns.clone(),
        };
        let controller = dynamic_controller(MicrophoneModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
