use std::{rc::Rc, sync::Arc};

use wayle_battery::BatteryService;
use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

use crate::shell::bar::dropdowns::DropdownRegistry;

pub(crate) struct BatteryInit {
    pub settings: BarSettings,
    pub battery: Arc<BatteryService>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum BatteryMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum BatteryCmd {
    UpdateLabel(String),
    UpdateIcon(String),
}
