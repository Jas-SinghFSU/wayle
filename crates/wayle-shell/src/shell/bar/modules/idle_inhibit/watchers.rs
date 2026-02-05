use relm4::ComponentSender;
use wayle_common::watch;
use wayle_config::schemas::modules::IdleInhibitConfig;

use super::{IdleInhibitModule, messages::IdleInhibitCmd};
use crate::services::idle_inhibit::IdleInhibitState;

pub(super) fn spawn_config_watchers(
    sender: &ComponentSender<IdleInhibitModule>,
    config: &IdleInhibitConfig,
) {
    let icon_inactive = config.icon_inactive.clone();
    let icon_active = config.icon_active.clone();
    let format = config.format.clone();

    watch!(
        sender,
        [icon_inactive.watch(), icon_active.watch(), format.watch()],
        |out| {
            let _ = out.send(IdleInhibitCmd::ConfigChanged);
        }
    );
}

pub(super) fn spawn_state_watchers(
    sender: &ComponentSender<IdleInhibitModule>,
    state: &IdleInhibitState,
) {
    let active = state.active.clone();
    let duration_mins = state.duration_mins.clone();
    let remaining_secs = state.remaining_secs.clone();

    watch!(
        sender,
        [
            active.watch(),
            duration_mins.watch(),
            remaining_secs.watch()
        ],
        |out| {
            let _ = out.send(IdleInhibitCmd::StateChanged);
        }
    );
}
