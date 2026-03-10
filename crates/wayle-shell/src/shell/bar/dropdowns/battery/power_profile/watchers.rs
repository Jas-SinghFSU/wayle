use std::sync::Arc;

use relm4::ComponentSender;
use tokio_util::sync::CancellationToken;
use wayle_common::{Property, watch, watch_cancellable};
use wayle_power_profiles::PowerProfilesService;

use super::{PowerProfileSection, messages::PowerProfileCmd};

pub(super) fn spawn_availability(
    sender: &ComponentSender<PowerProfileSection>,
    property: &Property<Option<Arc<PowerProfilesService>>>,
) {
    let property = property.clone();
    watch!(sender, [property.watch()], |out| {
        match property.get() {
            Some(service) => {
                let _ = out.send(PowerProfileCmd::ServiceAvailable(service));
            }
            None => {
                let _ = out.send(PowerProfileCmd::ServiceUnavailable);
            }
        }
    });
}

pub(super) fn spawn_profile_watchers(
    sender: &ComponentSender<PowerProfileSection>,
    service: &Arc<PowerProfilesService>,
    token: CancellationToken,
) {
    let active_profile = service.power_profiles.active_profile.clone();
    watch_cancellable!(sender, token.clone(), [active_profile.watch()], |out| {
        let _ = out.send(PowerProfileCmd::ProfileChanged(active_profile.get()));
    });

    let profiles = service.power_profiles.profiles.clone();
    watch_cancellable!(sender, token, [profiles.watch()], |out| {
        let available: Vec<_> = profiles
            .get()
            .into_iter()
            .map(|profile| profile.profile)
            .collect();
        let _ = out.send(PowerProfileCmd::AvailableProfilesChanged(available));
    });
}
