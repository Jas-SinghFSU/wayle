use std::time::Duration;

use relm4::ComponentSender;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use wayle_common::watch;
use wayle_config::schemas::modules::ClockConfig;

use super::{ClockModule, helpers::format_time, messages::ClockCmd};

pub(super) fn spawn_watchers(sender: &ComponentSender<ClockModule>, clock: &ClockConfig) {
    let format = clock.format.clone();
    let tick = interval(Duration::from_secs(1));
    let interval_stream = IntervalStream::new(tick);

    watch!(sender, [interval_stream], |out| {
        let formatted_time = format_time(&format.get());
        let _ = out.send(ClockCmd::UpdateTime(formatted_time));
    });

    let icon_name = clock.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(ClockCmd::UpdateIcon(icon_name.get().clone()));
    });

    let tooltip = clock.tooltip.clone();
    watch!(sender, [tooltip.watch()], |out| {
        let _ = out.send(ClockCmd::UpdateTooltip(tooltip.get().clone()));
    });
}
