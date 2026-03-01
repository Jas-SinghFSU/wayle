use relm4::prelude::*;

use super::{BluetoothDropdown, messages::BluetoothDropdownInit};
use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let bluetooth = services.bluetooth.clone()?;
        let config = services.config.clone();

        let init = BluetoothDropdownInit { bluetooth, config };
        let controller = BluetoothDropdown::builder().launch(init).detach();

        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}
