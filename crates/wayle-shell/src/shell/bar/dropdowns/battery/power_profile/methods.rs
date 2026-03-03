use relm4::prelude::*;
use tracing::warn;
use wayle_power_profiles::types::profile::PowerProfile;

use super::{PowerProfileSection, messages::PowerProfileCmd};

impl PowerProfileSection {
    pub(super) fn select_profile(&mut self, profile: PowerProfile, sender: &ComponentSender<Self>) {
        let Some(service) = self.power_profiles.clone() else {
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
}
