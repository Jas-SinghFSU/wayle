use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use relm4::ComponentSender;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use wayle_common::watch;
use wayle_config::schemas::modules::ClockConfig;

use super::{ClockModule, helpers::format_time, messages::ClockCmd};

pub(super) fn spawn_watchers(sender: &ComponentSender<ClockModule>, clock: &ClockConfig) {
    let interval_stream = IntervalStream::new(interval(Duration::from_secs(1)));
    let prev_label = Arc::new(Mutex::new(format_time(&clock.format.get())));

    let format = clock.format.clone();
    let prev = Arc::clone(&prev_label);
    watch!(sender, [interval_stream], |out| {
        let label = format_time(&format.get());
        let mut prev = prev.lock().unwrap_or_else(|poison| poison.into_inner());
        if *prev != label {
            *prev = label.clone();
            let _ = out.send(ClockCmd::UpdateTime(label));
        }
    });

    let format = clock.format.clone();
    let prev = Arc::clone(&prev_label);
    watch!(sender, [format.watch()], |out| {
        let label = format_time(&format.get());
        let mut prev = prev.lock().unwrap_or_else(|poison| poison.into_inner());
        if *prev != label {
            *prev = label.clone();
            let _ = out.send(ClockCmd::UpdateTime(label));
        }
    });

    let icon_name = clock.icon_name.clone();
    watch!(sender, [icon_name.watch()], |out| {
        let _ = out.send(ClockCmd::UpdateIcon(icon_name.get().clone()));
    });
}
