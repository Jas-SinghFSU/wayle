use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct DashboardInit {
    pub settings: BarSettings,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum DashboardMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum DashboardCmd {
    IconConfigChanged,
}
