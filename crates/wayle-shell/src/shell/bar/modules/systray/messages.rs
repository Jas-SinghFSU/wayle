use std::sync::Arc;

use wayle_common::ConfigProperty;
use wayle_config::ConfigService;
use wayle_systray::{SystemTrayService, core::item::TrayItem};

pub(crate) struct SystrayInit {
    pub is_vertical: ConfigProperty<bool>,
    pub systray: Arc<SystemTrayService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum SystrayMsg {}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum SystrayCmd {
    ItemsChanged(Vec<Arc<TrayItem>>),
    StylingChanged,
    OrientationChanged(bool),
}
