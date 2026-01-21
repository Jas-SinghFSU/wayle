mod clock;

use clock::{ClockInit, ClockModule};
use relm4::prelude::*;
use wayle_config::schemas::bar::BarModule;
use wayle_widgets::prelude::BarSettings;

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

pub(crate) fn create_module(module: &BarModule, settings: &BarSettings) -> ModuleController {
    match module {
        BarModule::Clock => {
            let init = ClockInit {
                settings: settings.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
        _ => {
            let init = ClockInit {
                settings: settings.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
    }
}
