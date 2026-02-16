use std::rc::Rc;

use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{BatteryInit, BatteryModule};
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
        let battery = require_service("battery", "battery", services.battery.clone())?;

        let init = BatteryInit {
            settings: settings.clone(),
            battery,
            config: services.config.clone(),
            dropdowns: dropdowns.clone(),
        };
        let controller = dynamic_controller(BatteryModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
