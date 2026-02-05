use std::time::Duration;

use relm4::ComponentSender;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use wayle_common::{watch, watch_async};
use wayle_config::schemas::modules::HyprsunsetConfig;

use super::{HyprsunsetModule, helpers, messages::HyprsunsetCmd};

pub(super) fn spawn_config_watchers(
    sender: &ComponentSender<HyprsunsetModule>,
    config: &HyprsunsetConfig,
) {
    let icon_off = config.icon_off.clone();
    let icon_on = config.icon_on.clone();
    let format = config.format.clone();

    watch!(
        sender,
        [icon_off.watch(), icon_on.watch(), format.watch()],
        |out| {
            let _ = out.send(HyprsunsetCmd::ConfigChanged);
        }
    );
}

pub(super) fn spawn_state_watcher(sender: &ComponentSender<HyprsunsetModule>) {
    let interval_stream = IntervalStream::new(interval(Duration::from_secs(1)));

    watch_async!(sender, [interval_stream], |out| async {
        let state = helpers::query_state().await;
        let _ = out.send(HyprsunsetCmd::StateChanged(state));
    });
}
