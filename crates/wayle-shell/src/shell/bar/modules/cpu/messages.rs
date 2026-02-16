use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct CpuInit {
    pub settings: BarSettings,
    pub sysinfo: Arc<SysinfoService>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
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
