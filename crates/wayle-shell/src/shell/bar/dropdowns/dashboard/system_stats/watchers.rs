use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_sysinfo::SysinfoService;
use wayle_widgets::watch_cancellable;

use super::{SystemStatsSection, messages::SystemStatsCmd};

pub(super) fn spawn(
    sender: &ComponentSender<SystemStatsSection>,
    sysinfo: &Arc<SysinfoService>,
    token: CancellationToken,
) {
    let cpu = sysinfo.cpu.clone();

    watch_cancellable!(sender, token.clone(), [cpu.watch()], |out| {
        let data = cpu.get();
        let _ = out.send(SystemStatsCmd::CpuChanged {
            usage: data.usage_percent,
            temp: data.temperature_celsius,
        });
    });

    let memory = sysinfo.memory.clone();

    watch_cancellable!(sender, token.clone(), [memory.watch()], |out| {
        let _ = out.send(SystemStatsCmd::MemoryChanged {
            usage: memory.get().usage_percent,
        });
    });

    let disks = sysinfo.disks.clone();

    watch_cancellable!(sender, token, [disks.watch()], |out| {
        let disk_data = disks.get();

        let root_usage = disk_data
            .iter()
            .find(|disk| disk.mount_point.as_os_str() == "/")
            .map_or(0.0, |disk| disk.usage_percent);
        let _ = out.send(SystemStatsCmd::DiskChanged { usage: root_usage });
    });
}
