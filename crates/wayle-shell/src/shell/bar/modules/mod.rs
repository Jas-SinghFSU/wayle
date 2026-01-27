mod battery;
mod bluetooth;
mod clock;
mod dashboard;
mod media;
mod microphone;
mod network;
mod notification;
mod systray;
mod volume;

use battery::{BatteryInit, BatteryModule};
use bluetooth::{BluetoothInit, BluetoothModule};
use clock::{ClockInit, ClockModule};
use dashboard::{DashboardInit, DashboardModule};
use media::{MediaInit, MediaModule};
use microphone::{MicrophoneInit, MicrophoneModule};
use network::{NetworkInit, NetworkModule};
use notification::{NotificationInit, NotificationModule};
use relm4::prelude::*;
use systray::{SystrayInit, SystrayModule};
use volume::{VolumeInit, VolumeModule};
use wayle_config::schemas::bar::{BarModule, ModuleRef};
use wayle_widgets::prelude::BarSettings;

pub(crate) struct ModuleInstance {
    pub(crate) controller: ModuleController,
    pub(crate) class: Option<String>,
}

pub(crate) enum ModuleController {
    Battery(Controller<BatteryModule>),
    Bluetooth(Controller<BluetoothModule>),
    Clock(Controller<ClockModule>),
    Dashboard(Controller<DashboardModule>),
    Media(Controller<MediaModule>),
    Microphone(Controller<MicrophoneModule>),
    Network(Controller<NetworkModule>),
    Notification(Controller<NotificationModule>),
    Systray(Controller<SystrayModule>),
    Volume(Controller<VolumeModule>),
}

impl ModuleController {
    pub(crate) fn widget(&self) -> &gtk::Box {
        match self {
            Self::Battery(c) => c.widget(),
            Self::Bluetooth(c) => c.widget(),
            Self::Clock(c) => c.widget(),
            Self::Dashboard(c) => c.widget(),
            Self::Media(c) => c.widget(),
            Self::Microphone(c) => c.widget(),
            Self::Network(c) => c.widget(),
            Self::Notification(c) => c.widget(),
            Self::Systray(c) => c.widget(),
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
        BarModule::Network => {
            let init = NetworkInit {
                settings: settings.clone(),
            };
            ModuleController::Network(NetworkModule::builder().launch(init).detach())
        }
        BarModule::Bluetooth => {
            let init = BluetoothInit {
                settings: settings.clone(),
            };
            ModuleController::Bluetooth(BluetoothModule::builder().launch(init).detach())
        }
        BarModule::Notifications => {
            let init = NotificationInit {
                settings: settings.clone(),
            };
            ModuleController::Notification(NotificationModule::builder().launch(init).detach())
        }
        BarModule::Systray => {
            let init = SystrayInit {
                is_vertical: settings.is_vertical.clone(),
            };
            ModuleController::Systray(SystrayModule::builder().launch(init).detach())
        }
        BarModule::Dashboard => {
            let init = DashboardInit {
                settings: settings.clone(),
            };
            ModuleController::Dashboard(DashboardModule::builder().launch(init).detach())
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
