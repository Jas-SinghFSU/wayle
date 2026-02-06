mod battery;
mod bluetooth;
mod clock;
mod compositor;
mod cpu;
mod dashboard;
mod hyprsunset;
mod idle_inhibit;
mod keybind_mode;
mod keyboard_input;
mod media;
mod microphone;
mod netstat;
mod network;
mod notification;
mod power;
mod ram;
mod separator;
mod storage;
mod systray;
mod volume;
mod weather;
mod window_title;
mod world_clock;

use battery::{BatteryInit, BatteryModule};
use bluetooth::{BluetoothInit, BluetoothModule};
use clock::{ClockInit, ClockModule};
use compositor::Compositor;
use cpu::{CpuInit, CpuModule};
use dashboard::{DashboardInit, DashboardModule};
use hyprsunset::{HyprsunsetInit, HyprsunsetModule};
use idle_inhibit::{IdleInhibitInit, IdleInhibitModule};
use keybind_mode::{HyprlandKeybindMode, KeybindModeInit};
use keyboard_input::{HyprlandKeyboardInput, KeyboardInputInit};
use media::{MediaInit, MediaModule};
use microphone::{MicrophoneInit, MicrophoneModule};
use netstat::{NetstatInit, NetstatModule};
use network::{NetworkInit, NetworkModule};
use notification::{NotificationInit, NotificationModule};
use power::{PowerInit, PowerModule};
use ram::{RamInit, RamModule};
use relm4::prelude::*;
use separator::{SeparatorInit, SeparatorModule};
use storage::{StorageInit, StorageModule};
use systray::{SystrayInit, SystrayModule};
use tracing::warn;
use volume::{VolumeInit, VolumeModule};
use wayle_config::schemas::bar::{BarModule, ModuleRef};
use wayle_widgets::prelude::BarSettings;
use weather::{WeatherInit, WeatherModule};
use window_title::{HyprlandWindowTitle, WindowTitleInit};
use world_clock::{WorldClockInit, WorldClockModule};

use crate::shell::services::ShellServices;

macro_rules! require_services {
    ($services:expr, $module:ident, [$($field:ident),+ $(,)?]) => {{
        $(
            let $field = match $services.$field.clone() {
                Some(svc) => svc,
                None => {
                    warn!(
                        module = stringify!($module),
                        service = stringify!($field),
                        "service unavailable, skipping module"
                    );
                    return None;
                }
            };
        )+
        ($($field),+)
    }};
}

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
    Hyprsunset(Controller<HyprsunsetModule>),
    IdleInhibit(Controller<IdleInhibitModule>),
    KeybindMode(Controller<HyprlandKeybindMode>),
    KeyboardInput(Controller<HyprlandKeyboardInput>),
    Media(Controller<MediaModule>),
    Microphone(Controller<MicrophoneModule>),
    Netstat(Controller<NetstatModule>),
    Network(Controller<NetworkModule>),
    Notification(Controller<NotificationModule>),
    Power(Controller<PowerModule>),
    Ram(Controller<RamModule>),
    Separator(Controller<SeparatorModule>),
    Storage(Controller<StorageModule>),
    Systray(Controller<SystrayModule>),
    Volume(Controller<VolumeModule>),
    Weather(Controller<WeatherModule>),
    WindowTitle(Controller<HyprlandWindowTitle>),
    WorldClock(Controller<WorldClockModule>),
}

impl ModuleController {
    pub(crate) fn widget(&self) -> &gtk::Box {
        match self {
            Self::Battery(c) => c.widget(),
            Self::Bluetooth(c) => c.widget(),
            Self::Clock(c) => c.widget(),
            Self::Cpu(c) => c.widget(),
            Self::Dashboard(c) => c.widget(),
            Self::Hyprsunset(c) => c.widget(),
            Self::IdleInhibit(c) => c.widget(),
            Self::KeybindMode(c) => c.widget(),
            Self::KeyboardInput(c) => c.widget(),
            Self::Media(c) => c.widget(),
            Self::Microphone(c) => c.widget(),
            Self::Netstat(c) => c.widget(),
            Self::Network(c) => c.widget(),
            Self::Notification(c) => c.widget(),
            Self::Power(c) => c.widget(),
            Self::Ram(c) => c.widget(),
            Self::Separator(c) => c.widget(),
            Self::Storage(c) => c.widget(),
            Self::Systray(c) => c.widget(),
            Self::Volume(c) => c.widget(),
            Self::Weather(c) => c.widget(),
            Self::WindowTitle(c) => c.widget(),
            Self::WorldClock(c) => c.widget(),
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn create_module(
    module_ref: &ModuleRef,
    settings: &BarSettings,
    services: &ShellServices,
) -> Option<ModuleInstance> {
    let module = module_ref.module();
    let class = module_ref.class().map(String::from);

    let controller = match module {
        BarModule::Battery => return create_battery_module(settings, services, class),
        BarModule::Bluetooth => return create_bluetooth_module(settings, services, class),
        BarModule::Hyprsunset => return create_hyprsunset_module(settings, services, class),
        BarModule::KeybindMode => return create_keybind_mode_module(settings, services, class),
        BarModule::KeyboardInput => return create_keyboard_input_module(settings, services, class),
        BarModule::Media => return create_media_module(settings, services, class),
        BarModule::Microphone => return create_microphone_module(settings, services, class),
        BarModule::Network => return create_network_module(settings, services, class),
        BarModule::Notifications => return create_notification_module(settings, services, class),
        BarModule::Systray => return create_systray_module(settings, services, class),
        BarModule::Volume => return create_volume_module(settings, services, class),
        BarModule::WindowTitle => return create_window_title_module(settings, services, class),
        BarModule::Clock => {
            let init = ClockInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
        BarModule::Dashboard => {
            let init = DashboardInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            ModuleController::Dashboard(DashboardModule::builder().launch(init).detach())
        }
        BarModule::IdleInhibit => {
            let init = IdleInhibitInit {
                settings: settings.clone(),
                idle_inhibit: services.idle_inhibit.clone(),
                config: services.config.clone(),
            };
            ModuleController::IdleInhibit(IdleInhibitModule::builder().launch(init).detach())
        }
        BarModule::Power => {
            let init = PowerInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            ModuleController::Power(PowerModule::builder().launch(init).detach())
        }
        BarModule::Cpu => {
            let init = CpuInit {
                settings: settings.clone(),
                sysinfo: services.sysinfo.clone(),
                config: services.config.clone(),
            };
            ModuleController::Cpu(CpuModule::builder().launch(init).detach())
        }
        BarModule::Ram => {
            let init = RamInit {
                settings: settings.clone(),
                sysinfo: services.sysinfo.clone(),
                config: services.config.clone(),
            };
            ModuleController::Ram(RamModule::builder().launch(init).detach())
        }
        BarModule::Storage => {
            let init = StorageInit {
                settings: settings.clone(),
                sysinfo: services.sysinfo.clone(),
                config: services.config.clone(),
            };
            ModuleController::Storage(StorageModule::builder().launch(init).detach())
        }
        BarModule::Netstat => {
            let init = NetstatInit {
                settings: settings.clone(),
                sysinfo: services.sysinfo.clone(),
                config: services.config.clone(),
            };
            ModuleController::Netstat(NetstatModule::builder().launch(init).detach())
        }
        BarModule::Separator => {
            let init = SeparatorInit {
                is_vertical: settings.is_vertical.clone(),
                config: services.config.clone(),
            };
            ModuleController::Separator(SeparatorModule::builder().launch(init).detach())
        }
        BarModule::Weather => {
            let init = WeatherInit {
                settings: settings.clone(),
                weather: services.weather.clone(),
                config: services.config.clone(),
            };
            ModuleController::Weather(WeatherModule::builder().launch(init).detach())
        }
        BarModule::WorldClock => {
            let init = WorldClockInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            ModuleController::WorldClock(WorldClockModule::builder().launch(init).detach())
        }
        _ => {
            let init = ClockInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            ModuleController::Clock(ClockModule::builder().launch(init).detach())
        }
    };

    Some(ModuleInstance { controller, class })
}

fn create_keybind_mode_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    match Compositor::detect() {
        Compositor::Hyprland => {
            let init = KeybindModeInit {
                settings: settings.clone(),
                hyprland: services.hyprland.clone(),
                config: services.config.clone(),
            };
            let controller =
                ModuleController::KeybindMode(HyprlandKeybindMode::builder().launch(init).detach());
            Some(ModuleInstance { controller, class })
        }
        Compositor::Unknown(name) => {
            warn!(compositor = %name, "unsupported compositor for keybind-mode");
            None
        }
    }
}

fn create_keyboard_input_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    match Compositor::detect() {
        Compositor::Hyprland => {
            let init = KeyboardInputInit {
                settings: settings.clone(),
                hyprland: services.hyprland.clone(),
                config: services.config.clone(),
            };
            let controller = ModuleController::KeyboardInput(
                HyprlandKeyboardInput::builder().launch(init).detach(),
            );
            Some(ModuleInstance { controller, class })
        }
        Compositor::Unknown(name) => {
            warn!(compositor = %name, "unsupported compositor for keyboard-input");
            None
        }
    }
}

fn create_window_title_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    match Compositor::detect() {
        Compositor::Hyprland => {
            let init = WindowTitleInit {
                settings: settings.clone(),
                hyprland: services.hyprland.clone(),
                config: services.config.clone(),
            };
            let controller =
                ModuleController::WindowTitle(HyprlandWindowTitle::builder().launch(init).detach());
            Some(ModuleInstance { controller, class })
        }
        Compositor::Unknown(name) => {
            warn!(compositor = %name, "unsupported compositor for window-title");
            None
        }
    }
}

fn create_hyprsunset_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    match Compositor::detect() {
        Compositor::Hyprland => {
            let init = HyprsunsetInit {
                settings: settings.clone(),
                config: services.config.clone(),
            };
            let controller =
                ModuleController::Hyprsunset(HyprsunsetModule::builder().launch(init).detach());
            Some(ModuleInstance { controller, class })
        }
        Compositor::Unknown(name) => {
            warn!(compositor = %name, "unsupported compositor for hyprsunset");
            None
        }
    }
}

fn create_battery_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let battery = require_services!(services, battery, [battery]);
    let init = BatteryInit {
        settings: settings.clone(),
        battery,
        config: services.config.clone(),
    };
    let controller = ModuleController::Battery(BatteryModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_media_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let media = require_services!(services, media, [media]);
    let init = MediaInit {
        settings: settings.clone(),
        media,
        config: services.config.clone(),
    };
    let controller = ModuleController::Media(MediaModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_volume_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let audio = require_services!(services, volume, [audio]);
    let init = VolumeInit {
        settings: settings.clone(),
        audio,
        config: services.config.clone(),
    };
    let controller = ModuleController::Volume(VolumeModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_microphone_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let audio = require_services!(services, microphone, [audio]);
    let init = MicrophoneInit {
        settings: settings.clone(),
        audio,
        config: services.config.clone(),
    };
    let controller =
        ModuleController::Microphone(MicrophoneModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_network_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let network = require_services!(services, network, [network]);
    let init = NetworkInit {
        settings: settings.clone(),
        network,
        config: services.config.clone(),
    };
    let controller = ModuleController::Network(NetworkModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_bluetooth_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let bluetooth = require_services!(services, bluetooth, [bluetooth]);
    let init = BluetoothInit {
        settings: settings.clone(),
        bluetooth,
        config: services.config.clone(),
    };
    let controller = ModuleController::Bluetooth(BluetoothModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_notification_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let notification = require_services!(services, notifications, [notification]);
    let init = NotificationInit {
        settings: settings.clone(),
        notification,
        config: services.config.clone(),
    };
    let controller =
        ModuleController::Notification(NotificationModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}

fn create_systray_module(
    settings: &BarSettings,
    services: &ShellServices,
    class: Option<String>,
) -> Option<ModuleInstance> {
    let systray = require_services!(services, systray, [systray]);
    let init = SystrayInit {
        is_vertical: settings.is_vertical.clone(),
        systray,
        config: services.config.clone(),
    };
    let controller = ModuleController::Systray(SystrayModule::builder().launch(init).detach());
    Some(ModuleInstance { controller, class })
}
