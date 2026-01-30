use std::sync::Arc;

use wayle_common::ConfigProperty;
use wayle_systray::core::item::TrayItem;

pub(crate) struct SystrayInit {
    pub is_vertical: ConfigProperty<bool>,
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
