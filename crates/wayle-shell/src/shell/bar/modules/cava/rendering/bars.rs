use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

use super::{MIN_BAR_HEIGHT, RenderParams, apply_color, fill_bar_rect};

pub(crate) fn draw_bars(
    cr: &cairo::Context,
    values: &[f64],
    canvas_height: f64,
    direction: CavaDirection,
    params: &RenderParams,
) {
    apply_color(cr, params);

    let bar_stride = params.bar_width + params.bar_spacing;

    for (bar_idx, &amplitude) in values.iter().enumerate() {
        let x = bar_idx as f64 * bar_stride;
        let bar_height = (amplitude * canvas_height).clamp(MIN_BAR_HEIGHT, canvas_height);

        fill_bar_rect(
            cr,
            x,
            bar_height,
            canvas_height,
            direction,
            params.bar_width,
        );
        let _ = cr.fill();
    }
}
