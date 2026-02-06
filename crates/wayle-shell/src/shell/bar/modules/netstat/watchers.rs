use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::schemas::modules::NetstatConfig;
use wayle_sysinfo::SysinfoService;

use super::{NetstatModule, helpers, messages::NetstatCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<NetstatModule>,
    config: &NetstatConfig,
    sysinfo: &Arc<SysinfoService>,
) {
    let format = config.format.clone();
    let interface = config.interface.clone();

    let sysinfo_network = sysinfo.clone();
    let sysinfo_format = sysinfo.clone();
    let sysinfo_interface = sysinfo.clone();

    watch!(sender, [sysinfo.network.watch()], |out| {
        let networks = sysinfo_network.network.get();
        let interface_config = interface.get();

        if let Some(net) = helpers::select_interface(&networks, &interface_config) {
            let label = helpers::format_label(&format.get(), net);
            let _ = out.send(NetstatCmd::UpdateLabel(label));
        }
    });

    let format_watch = config.format.clone();
    let interface_format = config.interface.clone();
    watch!(sender, [format_watch.watch()], |out| {
        let networks = sysinfo_format.network.get();
        let interface_config = interface_format.get();

        if let Some(net) = helpers::select_interface(&networks, &interface_config) {
            let label = helpers::format_label(&format_watch.get(), net);
            let _ = out.send(NetstatCmd::UpdateLabel(label));
        }
    });

    let interface_watch = config.interface.clone();
    let format_interface = config.format.clone();
    watch!(sender, [interface_watch.watch()], |out| {
        let networks = sysinfo_interface.network.get();

        if let Some(net) = helpers::select_interface(&networks, &interface_watch.get()) {
            let label = helpers::format_label(&format_interface.get(), net);
            let _ = out.send(NetstatCmd::UpdateLabel(label));
        }
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(NetstatCmd::UpdateIcon(icon_name.get().clone()));
    });
}
