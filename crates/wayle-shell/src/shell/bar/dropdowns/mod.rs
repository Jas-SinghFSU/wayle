mod audio;
mod registry;

pub(crate) use self::registry::{
    DropdownFactory, DropdownInstance, DropdownMargins, DropdownRegistry, dispatch_click,
};
use crate::shell::services::ShellServices;

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
}
