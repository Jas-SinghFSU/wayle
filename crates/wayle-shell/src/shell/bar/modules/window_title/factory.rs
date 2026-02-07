use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{HyprlandWindowTitle, WindowTitleInit};
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
        if !require_hyprland("window-title") {
            return None;
        }

        let init = WindowTitleInit {
            settings: settings.clone(),
            hyprland: services.hyprland.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(HyprlandWindowTitle::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
