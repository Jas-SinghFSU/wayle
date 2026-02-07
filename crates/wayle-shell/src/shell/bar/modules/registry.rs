use relm4::prelude::*;
use tracing::warn;
use wayle_widgets::prelude::BarSettings;

use super::compositor::Compositor;
use crate::shell::services::ShellServices;

pub(crate) struct ModuleInstance {
    pub(crate) controller: Box<dyn ModuleController>,
    pub(crate) class: Option<String>,
}

pub(crate) trait ModuleFactory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance>;
}

pub(crate) trait ModuleController {
    fn widget(&self) -> &gtk::Box;
}

struct ControllerHandle<C>
where
    C: Component<Root = gtk::Box>,
{
    controller: Controller<C>,
}

impl<C> ModuleController for ControllerHandle<C>
where
    C: Component<Root = gtk::Box> + 'static,
{
    fn widget(&self) -> &gtk::Box {
        self.controller.widget()
    }
}

pub(crate) fn dynamic_controller<C>(controller: Controller<C>) -> Box<dyn ModuleController>
where
    C: Component<Root = gtk::Box> + 'static,
{
    Box::new(ControllerHandle { controller })
}

pub(crate) fn require_service<T>(
    module: &'static str,
    service: &'static str,
    value: Option<T>,
) -> Option<T> {
    match value {
        Some(v) => Some(v),
        None => {
            warn!(module, service, "service unavailable, skipping module");
            None
        }
    }
}

pub(crate) fn require_hyprland(module: &'static str) -> bool {
    match Compositor::detect() {
        Compositor::Hyprland => true,
        Compositor::Unknown(name) => {
            warn!(module, compositor = %name, "unsupported compositor, skipping module");
            false
        }
    }
}
