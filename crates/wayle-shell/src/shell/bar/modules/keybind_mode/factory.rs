use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{HyprlandKeybindMode, KeybindModeInit};
use crate::shell::{
    bar::modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller, require_hyprland},
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        if !require_hyprland("keybind-mode") {
            return None;
        }

        let init = KeybindModeInit {
            settings: settings.clone(),
            hyprland: services.hyprland.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(HyprlandKeybindMode::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
