mod audio;
mod battery;
mod bluetooth;
mod calendar;
mod media;
mod network;
mod registry;
mod weather;

pub(crate) use self::registry::{
    DropdownFactory, DropdownInstance, DropdownRegistry, dispatch_click, dispatch_click_widget,
};
use crate::shell::services::ShellServices;

pub(crate) fn scaled_dimension(base: f32, scale: f32) -> i32 {
    (base * scale).round() as i32
}

macro_rules! register_dropdowns {
    ($($name:literal => $factory:ty),+ $(,)?) => {
        pub(crate) fn create(
            name: &str,
            services: &ShellServices,
        ) -> Option<DropdownInstance> {
            match name {
                $($name => <$factory as DropdownFactory>::create(services),)+
                _ => {
                    tracing::warn!(dropdown = name, "unknown dropdown type");
                    None
                }
            }
        }
    };
}

register_dropdowns! {
    "audio" => audio::Factory,
    "battery" => battery::Factory,
    "bluetooth" => bluetooth::Factory,
    "calendar" => calendar::Factory,
    "media" => media::Factory,
    "network" => network::Factory,
    "weather" => weather::Factory,
}
