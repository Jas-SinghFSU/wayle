use std::time::Duration;

use relm4::ComponentSender;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use wayle_common::watch;
use wayle_config::schemas::modules::WorldClockConfig;

use super::{WorldClockModule, helpers::format_world_clock, messages::WorldClockCmd};

pub(super) fn spawn_watchers(
    sender: &ComponentSender<WorldClockModule>,
    config: &WorldClockConfig,
) {
    let format = config.format.clone();
    let tick = interval(Duration::from_secs(1));
    let interval_stream = IntervalStream::new(tick);

    watch!(sender, [interval_stream], |out| {
        let label = format_world_clock(&format.get());
        let _ = out.send(WorldClockCmd::UpdateLabel(label));
    });

    let format = config.format.clone();
    watch!(sender, [format.watch()], |out| {
        let label = format_world_clock(&format.get());
        let _ = out.send(WorldClockCmd::UpdateLabel(label));
    });

    let icon_name = config.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(WorldClockCmd::UpdateIcon(icon_name.get().clone()));
    });
}
