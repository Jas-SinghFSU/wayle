use std::time::Duration;

use relm4::ComponentSender;
use wayle_common::{services, watch};
use wayle_config::schemas::modules::CpuConfig;
use wayle_sysinfo::SysinfoService;

use super::{CpuModule, helpers::format_label, messages::CpuCmd};

pub(super) fn spawn_watchers(sender: &ComponentSender<CpuModule>, config: &CpuConfig) {
    let sysinfo = services::get::<SysinfoService>();
    let format = config.format.clone();

    let sysinfo_cpu = sysinfo.clone();
    watch!(sender, [sysinfo.cpu.watch()], |out| {
        let cpu = sysinfo_cpu.cpu.get();
        let label = format_label(&format.get(), &cpu);
        let _ = out.send(CpuCmd::UpdateLabel(label));
    });

    let format_watch = config.format.clone();
    let sysinfo_format = sysinfo.clone();
    watch!(sender, [format_watch.watch()], |out| {
        let cpu = sysinfo_format.cpu.get();
        let label = format_label(&format_watch.get(), &cpu);
        let _ = out.send(CpuCmd::UpdateLabel(label));
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
    watch!(sender, [poll_interval.watch()], |_out| {
        sysinfo.set_cpu_interval(Duration::from_millis(poll_interval.get()));
    });
}
