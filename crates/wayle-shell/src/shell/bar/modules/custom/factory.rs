use std::rc::Rc;

use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{CustomInit, CustomModule};
use crate::shell::{
    bar::{
        dropdowns::DropdownRegistry,
        modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller},
    },
    services::ShellServices,
};

pub(crate) struct Factory;

impl Factory {
    pub fn create_for_id(
        id: &str,
        settings: &BarSettings,
        services: &ShellServices,
        dropdowns: &Rc<DropdownRegistry>,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let config = services.config.config();
        let definitions = config.modules.custom.get();
        let definition = definitions.iter().find(|def| def.id == id)?;

        let init = CustomInit {
            settings: settings.clone(),
            config: services.config.clone(),
            definition: definition.clone(),
            dropdowns: dropdowns.clone(),
        };
        let controller = dynamic_controller(CustomModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}

impl ModuleFactory for Factory {
    fn create(
        _settings: &BarSettings,
        _services: &ShellServices,
        _dropdowns: &Rc<DropdownRegistry>,
        _class: Option<String>,
    ) -> Option<ModuleInstance> {
        None
    }
}
