use relm4::{ComponentController, Controller};
use wayle_widgets::primitives::progress_ring::{ProgressRing, ProgressRingMsg};

use super::helpers;

const PERCENT_DIVISOR: f32 = 100.0;
const MAX_TEMP_DISPLAY: f32 = 100.0;

pub(super) fn update_usage_ring(ring: &Controller<ProgressRing>, usage: f32) {
    let fraction = (usage / PERCENT_DIVISOR) as f64;

    ring.emit(ProgressRingMsg::SetFraction(fraction));
    ring.emit(ProgressRingMsg::SetLabel(format!("{usage:.0}%")));
    ring.emit(ProgressRingMsg::SetColor(helpers::threshold_color(usage)));
}

pub(super) fn update_temp_ring(ring: &Controller<ProgressRing>, celsius: f32) {
    let fraction = (celsius / MAX_TEMP_DISPLAY).clamp(0.0, 1.0) as f64;

    ring.emit(ProgressRingMsg::SetFraction(fraction));
    ring.emit(ProgressRingMsg::SetLabel(format!("{celsius:.0}\u{00b0}")));
    ring.emit(ProgressRingMsg::SetColor(helpers::temp_color(celsius)));
}
