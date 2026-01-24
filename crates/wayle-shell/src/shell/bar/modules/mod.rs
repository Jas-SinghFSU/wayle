mod battery;
mod clock;
mod media;
mod microphone;
mod volume;

use battery::{BatteryInit, BatteryModule};
use clock::{ClockInit, ClockModule};
use media::{MediaInit, MediaModule};
use microphone::{MicrophoneInit, MicrophoneModule};
use relm4::prelude::*;
use volume::{VolumeInit, VolumeModule};
use wayle_config::schemas::bar::{BarModule, ModuleRef};
use wayle_widgets::prelude::BarSettings;

pub(crate) struct ModuleInstance {
    pub(crate) controller: ModuleController,
    pub(crate) class: Option<String>,
}

pub(crate) enum ModuleController {
    Battery(Controller<BatteryModule>),
    Clock(Controller<ClockModule>),
    Media(Controller<MediaModule>),
    Microphone(Controller<MicrophoneModule>),
    Volume(Controller<VolumeModule>),
}

impl ModuleController {
    pub(crate) fn widget(&self) -> &gtk::Box {
        match self {
            Self::Battery(c) => c.widget(),
            Self::Clock(c) => c.widget(),
            Self::Media(c) => c.widget(),
            Self::Microphone(c) => c.widget(),
            Self::Volume(c) => c.widget(),
        }
    }
}

pub(crate) fn create_module(module_ref: &ModuleRef, settings: &BarSettings) -> ModuleInstance {
    let module = module_ref.module();
    let class = module_ref.class().map(String::from);

    let controller = match module {
        BarModule::Battery => {
            let init = BatteryInit {
                settings: settings.clone(),
            };
            ModuleController::Battery(BatteryModule::builder().launch(init).detach())
        }
        BarModule::Clock => {
            let init = ClockInit {
                settings: settings.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
        BarModule::Media => {
            let init = MediaInit {
                settings: settings.clone(),
            };
            ModuleController::Media(MediaModule::builder().launch(init).detach())
        }
        BarModule::Volume => {
            let init = VolumeInit {
                settings: settings.clone(),
            };
            ModuleController::Volume(VolumeModule::builder().launch(init).detach())
        }
        BarModule::Microphone => {
            let init = MicrophoneInit {
                settings: settings.clone(),
            };
            ModuleController::Microphone(MicrophoneModule::builder().launch(init).detach())
        }
        _ => {
            let init = ClockInit {
                settings: settings.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
    };

    ModuleInstance { controller, class }
}
