use std::{sync::Arc, time::Duration};

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::ConfigService;

use super::{CalendarDropdown, helpers, messages::CalendarDropdownCmd};

const TICK_INTERVAL: Duration = Duration::from_secs(1);

pub(super) fn spawn(sender: &ComponentSender<CalendarDropdown>, config: &Arc<ConfigService>) {
    spawn_scale_watcher(sender, config);
    spawn_time_tick(sender);
    spawn_format_watcher(sender, config);
    spawn_show_seconds_watcher(sender, config);
}

fn spawn_scale_watcher(sender: &ComponentSender<CalendarDropdown>, config: &Arc<ConfigService>) {
    let scale = config.config().styling.scale.clone();

    watch!(sender, [scale.watch()], |out| {
        let _ = out.send(CalendarDropdownCmd::ScaleChanged(scale.get().value()));
    });
}

fn spawn_time_tick(sender: &ComponentSender<CalendarDropdown>) {
    sender.command(|out, shutdown| async move {
        let shutdown_fut = shutdown.wait();
        tokio::pin!(shutdown_fut);

        loop {
            tokio::select! {
                () = &mut shutdown_fut => break,
                () = tokio::time::sleep(TICK_INTERVAL) => {
                    let _ = out.send(CalendarDropdownCmd::TimeTick);
                }
            }
        }
    });
}

fn spawn_format_watcher(sender: &ComponentSender<CalendarDropdown>, config: &Arc<ConfigService>) {
    let format_prop = config.config().modules.clock.format.clone();

    watch!(sender, [format_prop.watch()], |out| {
        let use_12h = helpers::is_12h_format(&format_prop.get());
        let _ = out.send(CalendarDropdownCmd::FormatChanged(use_12h));
    });
}

fn spawn_show_seconds_watcher(
    sender: &ComponentSender<CalendarDropdown>,
    config: &Arc<ConfigService>,
) {
    let show_seconds = config.config().modules.clock.dropdown_show_seconds.clone();

    watch!(sender, [show_seconds.watch()], |out| {
        let _ = out.send(CalendarDropdownCmd::ShowSecondsChanged(show_seconds.get()));
    });
}
