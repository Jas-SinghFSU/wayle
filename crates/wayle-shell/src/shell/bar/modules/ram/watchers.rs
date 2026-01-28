use relm4::ComponentSender;
use wayle_common::{services, watch};
use wayle_config::schemas::modules::RamConfig;
use wayle_sysinfo::SysinfoService;

use super::{RamModule, helpers::format_label, messages::RamCmd};

pub(super) fn spawn_watchers(sender: &ComponentSender<RamModule>, config: &RamConfig) {
    let sysinfo = services::get::<SysinfoService>();
    let format = config.format.clone();

    let sysinfo_memory = sysinfo.clone();
    let sysinfo_format = sysinfo.clone();

    watch!(sender, [sysinfo.memory.watch()], |out| {
        let mem = sysinfo_memory.memory.get();
        let label = format_label(&format.get(), &mem);
        let _ = out.send(RamCmd::UpdateLabel(label));
    });

    let format_watch = config.format.clone();
    watch!(sender, [format_watch.watch()], |out| {
        let mem = sysinfo_format.memory.get();
        let label = format_label(&format_watch.get(), &mem);
        let _ = out.send(RamCmd::UpdateLabel(label));
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(RamCmd::UpdateIcon(icon_name.get().clone()));
    });
}
