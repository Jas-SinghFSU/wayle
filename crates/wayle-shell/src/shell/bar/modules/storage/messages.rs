use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct StorageInit {
    pub settings: BarSettings,
    pub sysinfo: Arc<SysinfoService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum StorageMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum StorageCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
