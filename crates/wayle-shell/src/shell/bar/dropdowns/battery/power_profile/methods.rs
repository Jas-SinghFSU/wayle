use std::sync::Arc;

use relm4::prelude::*;
use tracing::warn;
use wayle_power_profiles::{PowerProfilesService, types::profile::PowerProfile};

use super::{PowerProfileSection, messages::PowerProfileCmd, watchers};

impl PowerProfileSection {
    pub(super) fn select_profile(&mut self, profile: PowerProfile, sender: &ComponentSender<Self>) {
        let Some(service) = self.power_profiles.get() else {
            return;
        };

        self.active_profile = profile;

        sender.oneshot_command(async move {
            if let Err(err) = service.power_profiles.set_active_profile(profile).await {
                warn!(error = %err, "power profile switch failed");
            }
            PowerProfileCmd::ProfileChanged(profile)
        });
    }

    pub(super) fn apply_service(
        &mut self,
        sender: &ComponentSender<Self>,
        service: &Arc<PowerProfilesService>,
    ) {
        self.active_profile = service.power_profiles.active_profile.get();

        let available: Vec<_> = service
            .power_profiles
            .profiles
            .get()
            .into_iter()
            .map(|profile| profile.profile)
            .collect();
        self.has_saver = available.contains(&PowerProfile::PowerSaver);
        self.has_balanced = available.contains(&PowerProfile::Balanced);
        self.has_performance = available.contains(&PowerProfile::Performance);

        let token = self.profile_token.reset();
        watchers::spawn_profile_watchers(sender, service, token);
    }
}
