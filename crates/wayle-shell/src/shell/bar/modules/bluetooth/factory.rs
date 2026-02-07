use relm4::prelude::*;
use wayle_widgets::prelude::BarSettings;

use super::{BluetoothInit, BluetoothModule};
use crate::shell::{
    bar::modules::registry::{ModuleFactory, ModuleInstance, dynamic_controller, require_service},
    services::ShellServices,
};

pub(crate) struct Factory;

impl ModuleFactory for Factory {
    fn create(
        settings: &BarSettings,
        services: &ShellServices,
        class: Option<String>,
    ) -> Option<ModuleInstance> {
        let bluetooth = require_service("bluetooth", "bluetooth", services.bluetooth.clone())?;

        let init = BluetoothInit {
            settings: settings.clone(),
            bluetooth,
            config: services.config.clone(),
        };
        let controller = dynamic_controller(BluetoothModule::builder().launch(init).detach());
        Some(ModuleInstance { controller, class })
    }
}
