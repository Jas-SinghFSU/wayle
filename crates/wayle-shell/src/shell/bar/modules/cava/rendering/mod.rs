mod bars;
mod peaks;
mod wave;

use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

pub(super) use self::{bars::draw_bars, peaks::draw_peak_bars, wave::draw_wave};
use super::helpers::DrawConfig;

const MIN_BAR_HEIGHT: f64 = 2.0;

fn apply_color(cr: &cairo::Context, config: &DrawConfig) {
    let color = &config.color;
    cr.set_source_rgba(color.red, color.green, color.blue, color.alpha);
}

fn bar_y(direction: CavaDirection, bar_height: f64, canvas_height: f64) -> f64 {
    match direction {
        CavaDirection::Normal => canvas_height - bar_height,
        CavaDirection::Reverse => 0.0,
        CavaDirection::Mirror => (canvas_height - bar_height) / 2.0,
    }
}

fn draw_directed_bar(
    cr: &cairo::Context,
    x: f64,
    bar_height: f64,
    canvas_height: f64,
    direction: CavaDirection,
    bar_width: f64,
) {
    let y = bar_y(direction, bar_height, canvas_height);
    cr.rectangle(x, y, bar_width, bar_height);
}
