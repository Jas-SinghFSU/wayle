use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{HyprlandWorkspaces, WorkspacesInit};
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
        if !require_hyprland("hyprland-workspaces") {
            return None;
        }

        let init = WorkspacesInit {
            settings: settings.clone(),
            hyprland: services.hyprland.clone(),
            config: services.config.clone(),
        };
        let controller = dynamic_controller(HyprlandWorkspaces::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
