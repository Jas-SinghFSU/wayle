use std::{sync::Arc, time::Duration};

use relm4::ComponentSender;
use wayle_config::schemas::modules::CpuConfig;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::watch;

use super::{CpuModule, helpers::format_label, messages::CpuCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<CpuModule>,
    config: &CpuConfig,
    sysinfo: &Arc<SysinfoService>,
    has_barchart: bool,
) {
    let format = config.format.clone();

    let sysinfo_cpu = sysinfo.clone();
    watch!(sender, [sysinfo.cpu.watch()], |out| {
        let cpu = sysinfo_cpu.cpu.get();
        if has_barchart {
            let core_values: Vec<f64> = cpu
                .cores
                .iter()
                .map(|core| (core.usage_percent as f64) / 100.0)
                .collect();
            let _ = out.send(CpuCmd::UpdateBarchart(core_values));
        } else {
            let label = format_label(&format.get(), &cpu);
            let _ = out.send(CpuCmd::UpdateLabel(label));
        }
    });

    let format_watch = config.format.clone();
    let sysinfo_format = sysinfo.clone();
    watch!(sender, [format_watch.watch()], |out| {
        let cpu = sysinfo_format.cpu.get();
        if !has_barchart {
            let label = format_label(&format_watch.get(), &cpu);
            let _ = out.send(CpuCmd::UpdateLabel(label));
        }
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(CpuCmd::UpdateIcon(icon_name.get().clone()));
    });

    let temp_sensor = config.temp_sensor.clone();
    let sysinfo_sensor = sysinfo.clone();
    watch!(sender, [temp_sensor.watch()], |_out| {
        sysinfo_sensor.set_cpu_temp_sensor(&temp_sensor.get());
    });

    let poll_interval = config.poll_interval_ms.clone();
    let sysinfo_interval = sysinfo.clone();
    watch!(sender, [poll_interval.watch()], |_out| {
        sysinfo_interval.set_cpu_interval(Duration::from_millis(poll_interval.get()));
    });

    // Watch barchart config properties (only relevant when barchart is active)
    if has_barchart {
        let bar_width = config.barchart_bar_width.clone();
        let bar_gap = config.barchart_bar_gap.clone();
        let direction = config.barchart_direction.clone();
        let color = config.barchart_color.clone();
        let padding = config.barchart_internal_padding.clone();
        let sysinfo_redraw = sysinfo.clone();

        watch!(
            sender,
            [
                bar_width.watch(),
                bar_gap.watch(),
                direction.watch(),
                color.watch(),
                padding.watch()
            ],
            |out| {
                // Trigger a redraw with current CPU data
                let cpu = sysinfo_redraw.cpu.get();
                let core_values: Vec<f64> = cpu
                    .cores
                    .iter()
                    .map(|core| (core.usage_percent as f64) / 100.0)
                    .collect();
                let _ = out.send(CpuCmd::UpdateBarchart(core_values));
            }
        );
    }
}
