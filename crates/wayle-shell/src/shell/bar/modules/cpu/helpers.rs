use std::{cell::Cell, rc::Rc, sync::Arc};

use gtk4::prelude::*;
use serde_json::json;
use wayle_config::ConfigService;
use wayle_sysinfo::types::CpuData;
use wayle_widgets::primitives::barchart::{self, BarchartParams};

use super::super::shared;

/// Checks if the format string contains the barchart directive.
pub(super) fn has_barchart_directive(format: &str) -> bool {
    format.contains("{{ barchart }}")
}

pub(super) fn setup_barchart_draw_func(
    drawing_area: &gtk4::DrawingArea,
    core_values: &Rc<Cell<Vec<f64>>>,
    config: &Arc<ConfigService>,
) {
    let values = core_values.clone();
    let config_clone = config.clone();

    drawing_area.set_draw_func(move |_area, cr: &gtk4::cairo::Context, _width, height| {
        let pixel_height = height as f64;

        let core_data = values.take();
        if core_data.is_empty() {
            values.set(core_data);
            return;
        }

        // Read config values fresh on each draw
        let full_config = config_clone.config();
        let cpu_config = &full_config.modules.cpu;

        let bar_width = cpu_config.barchart_bar_width.get() as f64;
        let bar_spacing = cpu_config.barchart_bar_gap.get() as f64;
        let bar_scale = full_config.bar.scale.get().value();
        let padding_rem = cpu_config.barchart_internal_padding.get().value();
        let horizontal_padding = shared::rem_to_px(padding_rem, bar_scale);
        let direction = cpu_config.barchart_direction.get();
        let color = cpu_config.barchart_color.get();

        cr.translate(horizontal_padding, 0.0);

        let fill_color = shared::resolve_rgba(&color, &config_clone);

        let params = BarchartParams {
            bar_width,
            bar_spacing,
            fill_color,
        };

        barchart::draw_barchart(cr, &core_data, pixel_height, direction, &params);

        values.set(core_data);
    });
}

pub(super) fn update_barchart_size(
    drawing_area: &gtk4::DrawingArea,
    num_cores: usize,
    config: &Arc<ConfigService>,
    is_vertical: bool,
) {
    let full_config = config.config();
    let cpu_config = &full_config.modules.cpu;
    let bar_width = cpu_config.barchart_bar_width.get();
    let bar_gap = cpu_config.barchart_bar_gap.get();
    let bar_scale = full_config.bar.scale.get().value();
    let padding_rem = cpu_config.barchart_internal_padding.get().value();
    let padding_px = shared::rem_to_px(padding_rem, bar_scale);

    let length =
        barchart::calculate_widget_length(num_cores as u16, bar_width, bar_gap, padding_px);
println!("REQUESTING SIZE length {}", length);
    if is_vertical {
        drawing_area.set_size_request(-1, length);
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(false);
    } else {
        drawing_area.set_size_request(length, -1);
        drawing_area.set_vexpand(true);
        drawing_area.set_hexpand(true);
    }
}

/// Formats a CPU label using Jinja2 template syntax.
///
/// ## Variables
///
/// - `{{ percent }}` - CPU usage (00-100, zero-padded)
/// - `{{ freq_ghz }}` - Frequency of the busiest core (highest usage)
/// - `{{ avg_freq_ghz }}` - Average frequency across cores
/// - `{{ max_freq_ghz }}` - Maximum frequency among cores
/// - `{{ temp_c }}` - Temperature in Celsius (zero-padded)
/// - `{{ temp_f }}` - Temperature in Fahrenheit (zero-padded)
pub(super) fn format_label(format: &str, cpu: &CpuData) -> String {
    let busiest_ghz = cpu.busiest_core_freq_mhz as f64 / 1000.0;
    let avg_ghz = cpu.avg_frequency_mhz as f64 / 1000.0;
    let max_ghz = cpu.max_frequency_mhz as f64 / 1000.0;
    let temp_c = cpu.temperature_celsius.unwrap_or(0.0);
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;

    let ctx = json!({
        "percent": format!("{:02.0}", cpu.usage_percent),
        "freq_ghz": format!("{busiest_ghz:.1}"),
        "avg_freq_ghz": format!("{avg_ghz:.1}"),
        "max_freq_ghz": format!("{max_ghz:.1}"),
        "temp_c": format!("{temp_c:02.0}"),
        "temp_f": format!("{temp_f:02.0}"),
    });
    crate::template::render(format, ctx).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu_data(
        usage: f32,
        avg_mhz: u64,
        max_mhz: u64,
        busiest_mhz: u64,
        temp: Option<f32>,
    ) -> CpuData {
        CpuData {
            usage_percent: usage,
            avg_frequency_mhz: avg_mhz,
            max_frequency_mhz: max_mhz,
            busiest_core_freq_mhz: busiest_mhz,
            temperature_celsius: temp,
            cores: vec![],
        }
    }

    #[test]
    fn format_label_replaces_percent_placeholder() {
        let cpu = cpu_data(45.7, 3500, 4500, 4200, Some(55.0));
        let result = format_label("{{ percent }}%", &cpu);
        assert_eq!(result, "46%");
    }

    #[test]
    fn format_label_percent_pads_single_digits() {
        let cpu = cpu_data(5.2, 3500, 4500, 4200, Some(55.0));
        let result = format_label("{{ percent }}", &cpu);
        assert_eq!(result, "05");
    }

    #[test]
    fn format_label_replaces_freq_ghz_placeholder() {
        let cpu = cpu_data(50.0, 2900, 4500, 3800, Some(55.0));
        let result = format_label("{{ freq_ghz }} GHz", &cpu);
        assert_eq!(result, "3.8 GHz");
    }

    #[test]
    fn format_label_freq_ghz_rounds_correctly() {
        let cpu = cpu_data(50.0, 3000, 4500, 4750, Some(55.0));
        let result = format_label("{{ freq_ghz }}", &cpu);
        assert_eq!(result, "4.8");
    }

    #[test]
    fn format_label_replaces_avg_freq_ghz_placeholder() {
        let cpu = cpu_data(50.0, 2900, 4500, 4200, Some(55.0));
        let result = format_label("{{ avg_freq_ghz }} GHz", &cpu);
        assert_eq!(result, "2.9 GHz");
    }

    #[test]
    fn format_label_replaces_max_freq_ghz_placeholder() {
        let cpu = cpu_data(50.0, 2900, 4500, 4200, Some(55.0));
        let result = format_label("{{ max_freq_ghz }} GHz", &cpu);
        assert_eq!(result, "4.5 GHz");
    }

    #[test]
    fn format_label_avg_freq_ghz_rounds_correctly() {
        let cpu = cpu_data(50.0, 4750, 4750, 4750, Some(55.0));
        let result = format_label("{{ avg_freq_ghz }}", &cpu);
        assert_eq!(result, "4.8");
    }

    #[test]
    fn format_label_replaces_temp_c_placeholder() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(55.3));
        let result = format_label("{{ temp_c }}°C", &cpu);
        assert_eq!(result, "55°C");
    }

    #[test]
    fn format_label_temp_c_pads_single_digits() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(8.0));
        let result = format_label("{{ temp_c }}", &cpu);
        assert_eq!(result, "08");
    }

    #[test]
    fn format_label_replaces_temp_f_placeholder() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(100.0));
        let result = format_label("{{ temp_f }}°F", &cpu);
        assert_eq!(result, "212°F");
    }

    #[test]
    fn format_label_temp_f_converts_freezing_point() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(0.0));
        let result = format_label("{{ temp_f }}", &cpu);
        assert_eq!(result, "32");
    }

    #[test]
    fn format_label_with_no_temperature_uses_zero() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, None);
        let result = format_label("{{ temp_c }}°C / {{ temp_f }}°F", &cpu);
        assert_eq!(result, "00°C / 32°F");
    }

    #[test]
    fn format_label_with_multiple_placeholders() {
        let cpu = cpu_data(75.0, 2900, 4500, 4200, Some(65.0));
        let result = format_label(
            "{{ percent }}% @ {{ max_freq_ghz }}GHz (avg {{ avg_freq_ghz }})",
            &cpu,
        );
        assert_eq!(result, "75% @ 4.5GHz (avg 2.9)");
    }

    #[test]
    fn format_label_with_no_placeholders_returns_unchanged() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(55.0));
        let result = format_label("Static Text", &cpu);
        assert_eq!(result, "Static Text");
    }

    #[test]
    fn format_label_with_empty_format_returns_empty() {
        let cpu = cpu_data(50.0, 3500, 4500, 4200, Some(55.0));
        let result = format_label("", &cpu);
        assert_eq!(result, "");
    }
}
