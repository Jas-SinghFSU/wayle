use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct NetstatInit {
    pub settings: BarSettings,
    pub sysinfo: Arc<SysinfoService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum NetstatMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum NetstatCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
