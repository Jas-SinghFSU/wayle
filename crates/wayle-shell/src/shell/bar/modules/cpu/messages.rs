use std::sync::Arc;

use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct CpuInit {
    pub settings: BarSettings,
    pub sysinfo: Arc<SysinfoService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug)]
pub(crate) enum CpuMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum CpuCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
