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
mod registry;
mod separator;
mod storage;
mod systray;
mod volume;
mod weather;
mod window_title;
mod world_clock;

use tracing::warn;
use wayle_config::schemas::bar::{BarModule, ModuleRef};
use wayle_widgets::prelude::BarSettings;

pub(crate) use self::registry::{ModuleFactory, ModuleInstance};
use crate::shell::services::ShellServices;

macro_rules! register_modules {
    ($($variant:ident => $factory:ty),+ $(,)?) => {
        fn create_from_variant(
            module: BarModule,
            settings: &BarSettings,
            services: &ShellServices,
            class: Option<String>,
        ) -> Option<ModuleInstance> {
            match module {
                $(BarModule::$variant => <$factory as ModuleFactory>::create(settings, services, class),)+
                _ => {
                    warn!(?module, "module not implemented");
                    None
                }
            }
        }
    };
}

register_modules! {
    Battery => battery::Factory,
    Bluetooth => bluetooth::Factory,
    Clock => clock::Factory,
    Cpu => cpu::Factory,
    Dashboard => dashboard::Factory,
    Hyprsunset => hyprsunset::Factory,
    IdleInhibit => idle_inhibit::Factory,
    KeybindMode => keybind_mode::Factory,
    KeyboardInput => keyboard_input::Factory,
    Media => media::Factory,
    Microphone => microphone::Factory,
    Netstat => netstat::Factory,
    Network => network::Factory,
    Notifications => notification::Factory,
    Power => power::Factory,
    Ram => ram::Factory,
    Separator => separator::Factory,
    Storage => storage::Factory,
    Systray => systray::Factory,
    Volume => volume::Factory,
    Weather => weather::Factory,
    WindowTitle => window_title::Factory,
    WorldClock => world_clock::Factory,
}

pub(crate) fn create_module(
    module_ref: &ModuleRef,
    settings: &BarSettings,
    services: &ShellServices,
) -> Option<ModuleInstance> {
    let module = module_ref.module();
    let class = module_ref.class().map(String::from);
    create_from_variant(module, settings, services, class)
}
