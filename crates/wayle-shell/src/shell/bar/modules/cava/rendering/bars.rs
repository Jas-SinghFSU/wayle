use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

use super::{DrawConfig, MIN_BAR_HEIGHT, apply_color, draw_directed_bar};

pub(crate) fn draw_bars(
    cr: &cairo::Context,
    values: &[f64],
    canvas_height: f64,
    direction: CavaDirection,
    config: &DrawConfig,
) {
    apply_color(cr, config);

    let step = config.bar_width + config.bar_gap;

    for (index, &value) in values.iter().enumerate() {
        let x = index as f64 * step;
        let bar_height = (value * canvas_height).clamp(MIN_BAR_HEIGHT, canvas_height);

        draw_directed_bar(
            cr,
            x,
            bar_height,
            canvas_height,
            direction,
            config.bar_width,
        );
        let _ = cr.fill();
    }
}
