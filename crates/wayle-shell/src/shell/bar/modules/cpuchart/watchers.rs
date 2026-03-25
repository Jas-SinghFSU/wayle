use std::sync::Arc;

use relm4::ComponentSender;
use wayle_config::schemas::modules::CpuChartConfig;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::watch;

use super::{CpuChartModule, messages::CpuChartCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<CpuChartModule>,
    config: &CpuChartConfig,
    sysinfo: &Arc<SysinfoService>,
) {
    let sysinfo_cpu = sysinfo.clone();
    watch!(sender, [sysinfo.cpu.watch()], |out| {
        let cpu = sysinfo_cpu.cpu.get();
        let core_values: Vec<f64> = cpu
            .cores
            .iter()
            .map(|core| (core.usage_percent as f64) / 100.0)
            .collect();
        let _ = out.send(CpuChartCmd::UpdateChart(core_values));
    });

    let bar_width = config.bar_width.clone();
    let bar_gap = config.bar_gap.clone();
    let direction = config.direction.clone();
    let color = config.color.clone();
    let padding = config.internal_padding.clone();
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
            let cpu = sysinfo_redraw.cpu.get();
            let core_values: Vec<f64> = cpu
                .cores
                .iter()
                .map(|core| (core.usage_percent as f64) / 100.0)
                .collect();
            let _ = out.send(CpuChartCmd::UpdateChart(core_values));
        }
    );
}
