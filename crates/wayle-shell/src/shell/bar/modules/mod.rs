mod clock;

use clock::{ClockInit, ClockModule};
use relm4::prelude::*;
use wayle_common::ConfigProperty;
use wayle_config::schemas::bar::BarModule;

pub(crate) enum ModuleController {
    Clock(Controller<ClockModule>),
}

impl ModuleController {
    pub(crate) fn widget(&self) -> &gtk::Box {
        match self {
            Self::Clock(c) => c.widget(),
        }
    }
}

pub(crate) fn create_module(
    module: &BarModule,
    is_vertical: &ConfigProperty<bool>,
) -> ModuleController {
    match module {
        BarModule::Clock => {
            let init = ClockInit {
                is_vertical: is_vertical.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
        _ => {
            let init = ClockInit {
                is_vertical: is_vertical.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
    }
}
