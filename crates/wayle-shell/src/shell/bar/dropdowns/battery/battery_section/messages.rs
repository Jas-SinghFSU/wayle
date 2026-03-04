use std::sync::Arc;

use wayle_battery::BatteryService;

pub(crate) struct BatterySectionInit {
    pub battery: Arc<BatteryService>,
}

#[derive(Debug)]
pub(crate) enum BatterySectionInput {
    ChargeLimitToggled(bool),
}

#[derive(Debug)]
pub(crate) enum BatterySectionCmd {
    BatteryStateChanged,
}
