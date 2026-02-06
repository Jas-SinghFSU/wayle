use std::sync::Arc;

use wayle_battery::BatteryService;
use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

pub(crate) struct BatteryInit {
    pub settings: BarSettings,
    pub battery: Arc<BatteryService>,
    pub config: Arc<ConfigService>,
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
