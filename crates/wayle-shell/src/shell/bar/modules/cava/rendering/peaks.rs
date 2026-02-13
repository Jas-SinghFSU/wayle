use gtk4::cairo;
use wayle_config::schemas::modules::CavaDirection;

use super::{DrawConfig, MIN_BAR_HEIGHT, apply_color, draw_directed_bar};

const PEAK_CAP_HEIGHT: f64 = 2.0;
const PEAK_GRAVITY: f64 = 0.015;

/// Per-bar peak amplitude for decay tracking.
pub(crate) type PeakState = Vec<f64>;

pub(crate) fn draw_peak_bars(
    cr: &cairo::Context,
    values: &[f64],
    peaks: &mut PeakState,
    canvas_height: f64,
    direction: CavaDirection,
    config: &DrawConfig,
) {
    apply_color(cr, config);

    let step = config.bar_width + config.bar_gap;

    peaks.resize(values.len(), 0.0);

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

        update_peak(&mut peaks[index], value);

        let peak_height = peaks[index] * canvas_height;
        draw_peak_cap(
            cr,
            x,
            peak_height,
            bar_height,
            canvas_height,
            direction,
            config.bar_width,
        );
    }
}

fn update_peak(peak: &mut f64, current_value: f64) {
    if current_value >= *peak {
        *peak = current_value;
    } else {
        *peak = (*peak - PEAK_GRAVITY).max(0.0);
    }
}

fn draw_peak_cap(
    cr: &cairo::Context,
    x: f64,
    peak_height: f64,
    bar_height: f64,
    canvas_height: f64,
    direction: CavaDirection,
    bar_width: f64,
) {
    if peak_height <= bar_height {
        return;
    }

    let cap_height = PEAK_CAP_HEIGHT.min(canvas_height);

    match direction {
        CavaDirection::Normal => {
            cr.rectangle(
                x,
                canvas_height - peak_height - cap_height,
                bar_width,
                cap_height,
            );
        }
        CavaDirection::Reverse => {
            cr.rectangle(x, peak_height, bar_width, cap_height);
        }
        CavaDirection::Mirror => {
            let peak_half = peak_height / 2.0;
            let center = canvas_height / 2.0;

            cr.rectangle(x, center - peak_half - cap_height, bar_width, cap_height);
            let _ = cr.fill();
            cr.rectangle(x, center + peak_half, bar_width, cap_height);
        }
    }

    let _ = cr.fill();
}
