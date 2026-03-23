mod bars;
mod peaks;
mod wave;

use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

pub(super) use self::{bars::draw_bars, peaks::draw_peak_bars, wave::draw_wave};
use super::color::Rgba;

const MIN_BAR_HEIGHT: f64 = 2.0;

pub(super) struct RenderParams {
    pub bar_width: f64,
    pub bar_spacing: f64,
    pub fill_color: Rgba,
}

fn apply_color(cr: &cairo::Context, params: &RenderParams) {
    let color = &params.fill_color;
    cr.set_source_rgba(color.red, color.green, color.blue, color.alpha);
}

fn bar_origin_y(direction: CavaDirection, bar_height: f64, canvas_height: f64) -> f64 {
    match direction {
        CavaDirection::Normal => canvas_height - bar_height,
        CavaDirection::Reverse => 0.0,
        CavaDirection::Mirror => (canvas_height - bar_height) / 2.0,
    }
}

fn fill_bar_rect(
    cr: &cairo::Context,
    x: f64,
    bar_height: f64,
    canvas_height: f64,
    direction: CavaDirection,
    bar_width: f64,
) {
    let y = bar_origin_y(direction, bar_height, canvas_height);
    cr.rectangle(x, y, bar_width, bar_height);
}
