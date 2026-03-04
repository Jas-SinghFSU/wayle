use std::sync::Arc;

use relm4::ComponentSender;
use wayle_common::watch;
use wayle_power_profiles::PowerProfilesService;

use super::{PowerProfileSection, messages::PowerProfileCmd};

pub(super) fn spawn(
    sender: &ComponentSender<PowerProfileSection>,
    power_profiles: &Arc<PowerProfilesService>,
) {
    let active_profile = power_profiles.power_profiles.active_profile.clone();
    watch!(sender, [active_profile.watch()], |out| {
        let _ = out.send(PowerProfileCmd::ProfileChanged(active_profile.get()));
    });

    let profiles = power_profiles.power_profiles.profiles.clone();
    watch!(sender, [profiles.watch()], |out| {
        let available: Vec<_> = profiles
            .get()
            .into_iter()
            .map(|profile| profile.profile)
            .collect();
        let _ = out.send(PowerProfileCmd::AvailableProfilesChanged(available));
    });
}
