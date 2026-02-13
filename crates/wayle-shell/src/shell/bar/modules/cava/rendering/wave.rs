use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

use super::{DrawConfig, apply_color};

const MIN_WAVE_HEIGHT: f64 = 2.0;

pub(crate) fn draw_wave(
    cr: &cairo::Context,
    values: &[f64],
    canvas_width: f64,
    canvas_height: f64,
    direction: CavaDirection,
    config: &DrawConfig,
) {
    if values.is_empty() {
        return;
    }

    apply_color(cr, config);

    let count = values.len();
    let step = canvas_width / count as f64;
    let min_ratio = MIN_WAVE_HEIGHT / canvas_height;

    let value_to_y = |value: f64| -> f64 {
        let clamped = value.max(min_ratio);
        match direction {
            CavaDirection::Normal => canvas_height * (1.0 - clamped),
            CavaDirection::Reverse => canvas_height * clamped,
            CavaDirection::Mirror => canvas_height * (1.0 - clamped) / 2.0,
        }
    };

    trace_wave_curve(cr, values, step, &value_to_y);
    close_wave_path(
        cr,
        values,
        canvas_width,
        canvas_height,
        step,
        min_ratio,
        direction,
    );

    cr.close_path();
    let _ = cr.fill();
}

fn trace_wave_curve(
    cr: &cairo::Context,
    values: &[f64],
    step: f64,
    value_to_y: &dyn Fn(f64) -> f64,
) {
    cr.move_to(0.0, value_to_y(values[0]));

    for i in 1..values.len() {
        let x = i as f64 * step;
        let prev_x = (i - 1) as f64 * step;
        let control_x = (prev_x + x) / 2.0;

        cr.curve_to(
            control_x,
            value_to_y(values[i - 1]),
            control_x,
            value_to_y(values[i]),
            x,
            value_to_y(values[i]),
        );
    }
}

fn close_wave_path(
    cr: &cairo::Context,
    values: &[f64],
    canvas_width: f64,
    canvas_height: f64,
    step: f64,
    min_ratio: f64,
    direction: CavaDirection,
) {
    match direction {
        CavaDirection::Normal => {
            cr.line_to(canvas_width, canvas_height);
            cr.line_to(0.0, canvas_height);
        }
        CavaDirection::Reverse => {
            cr.line_to(canvas_width, 0.0);
            cr.line_to(0.0, 0.0);
        }
        CavaDirection::Mirror => {
            trace_mirror_bottom(cr, values, step, min_ratio, canvas_height);
        }
    }
}

fn trace_mirror_bottom(
    cr: &cairo::Context,
    values: &[f64],
    step: f64,
    min_ratio: f64,
    canvas_height: f64,
) {
    let count = values.len();
    let center = canvas_height / 2.0;

    for i in (0..count).rev() {
        let x = i as f64 * step;
        let clamped = values[i].max(min_ratio);
        let mirror_y = center + clamped * canvas_height / 2.0;

        if i == count - 1 {
            cr.line_to(x, mirror_y);
            continue;
        }

        let next_x = (i + 1) as f64 * step;
        let control_x = (x + next_x) / 2.0;
        let next_clamped = values[i + 1].max(min_ratio);
        let next_mirror_y = center + next_clamped * canvas_height / 2.0;

        cr.curve_to(control_x, next_mirror_y, control_x, mirror_y, x, mirror_y);
    }
}
