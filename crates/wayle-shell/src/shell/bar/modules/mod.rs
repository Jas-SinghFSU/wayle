mod battery;
mod bluetooth;
mod clock;
mod cpu;
mod dashboard;
mod media;
mod microphone;
mod netstat;
mod network;
mod notification;
mod ram;
mod separator;
mod storage;
mod systray;
mod volume;

use battery::{BatteryInit, BatteryModule};
use bluetooth::{BluetoothInit, BluetoothModule};
use clock::{ClockInit, ClockModule};
use cpu::{CpuInit, CpuModule};
use dashboard::{DashboardInit, DashboardModule};
use media::{MediaInit, MediaModule};
use microphone::{MicrophoneInit, MicrophoneModule};
use netstat::{NetstatInit, NetstatModule};
use network::{NetworkInit, NetworkModule};
use notification::{NotificationInit, NotificationModule};
use ram::{RamInit, RamModule};
use relm4::prelude::*;
use separator::{SeparatorInit, SeparatorModule};
use storage::{StorageInit, StorageModule};
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
    Cpu(Controller<CpuModule>),
    Dashboard(Controller<DashboardModule>),
    Media(Controller<MediaModule>),
    Microphone(Controller<MicrophoneModule>),
    Netstat(Controller<NetstatModule>),
    Network(Controller<NetworkModule>),
    Notification(Controller<NotificationModule>),
    Ram(Controller<RamModule>),
    Separator(Controller<SeparatorModule>),
    Storage(Controller<StorageModule>),
    Systray(Controller<SystrayModule>),
    Volume(Controller<VolumeModule>),
}

impl ModuleController {
    pub(crate) fn widget(&self) -> &gtk::Box {
        match self {
            Self::Battery(c) => c.widget(),
            Self::Bluetooth(c) => c.widget(),
            Self::Clock(c) => c.widget(),
            Self::Cpu(c) => c.widget(),
            Self::Dashboard(c) => c.widget(),
            Self::Media(c) => c.widget(),
            Self::Microphone(c) => c.widget(),
            Self::Netstat(c) => c.widget(),
            Self::Network(c) => c.widget(),
            Self::Notification(c) => c.widget(),
            Self::Ram(c) => c.widget(),
            Self::Separator(c) => c.widget(),
            Self::Storage(c) => c.widget(),
            Self::Systray(c) => c.widget(),
            Self::Volume(c) => c.widget(),
        }
    }
}

#[allow(clippy::too_many_lines)]
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
        BarModule::Cpu => {
            let init = CpuInit {
                settings: settings.clone(),
            };
            ModuleController::Cpu(CpuModule::builder().launch(init).detach())
        }
        BarModule::Ram => {
            let init = RamInit {
                settings: settings.clone(),
            };
            ModuleController::Ram(RamModule::builder().launch(init).detach())
        }
        BarModule::Storage => {
            let init = StorageInit {
                settings: settings.clone(),
            };
            ModuleController::Storage(StorageModule::builder().launch(init).detach())
        }
        BarModule::Netstat => {
            let init = NetstatInit {
                settings: settings.clone(),
            };
            ModuleController::Netstat(NetstatModule::builder().launch(init).detach())
        }
        BarModule::Separator => {
            let init = SeparatorInit {
                is_vertical: settings.is_vertical.clone(),
            };
            ModuleController::Separator(SeparatorModule::builder().launch(init).detach())
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
