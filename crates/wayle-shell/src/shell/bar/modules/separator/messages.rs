use std::sync::Arc;

use wayle_common::ConfigProperty;
use wayle_config::ConfigService;

pub(crate) struct SeparatorInit {
    pub is_vertical: ConfigProperty<bool>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum SeparatorCmd {
    StylingChanged,
    OrientationChanged(bool),
}
