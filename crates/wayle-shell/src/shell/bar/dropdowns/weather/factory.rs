use relm4::prelude::*;

use super::{WeatherDropdown, messages::WeatherDropdownInit};
use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let weather = services.weather.clone();
        let config = services.config.clone();

        let init = WeatherDropdownInit { weather, config };
        let controller = WeatherDropdown::builder().launch(init).detach();

        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}
