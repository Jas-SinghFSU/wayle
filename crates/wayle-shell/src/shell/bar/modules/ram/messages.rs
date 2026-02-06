use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct RamInit {
    pub settings: BarSettings,
    pub sysinfo: Arc<SysinfoService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum RamMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum RamCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
