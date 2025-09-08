use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use super::PowerProfiles;
use crate::services::{
    power_profiles::{
        error::Error,
        proxy::power_profiles::PowerProfilesProxy,
        types::profile::{PerformanceDegradationReason, PowerProfile, Profile, ProfileHold},
    },
    traits::ModelMonitoring,
};

impl ModelMonitoring for PowerProfiles {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = PowerProfilesProxy::new(&self.zbus_connection).await?;
        let weak_self = Arc::downgrade(&self);
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::OperationFailed {
                operation: "start_monitoring",
                reason: String::from("A cancellation_token was not found."),
            });
        };

        monitor_power_profiles(weak_self, proxy, cancellation_token.clone()).await
    }
}

async fn monitor_power_profiles(
    weak_power_profiles: Weak<PowerProfiles>,
    proxy: PowerProfilesProxy<'static>,
    cancel_token: CancellationToken,
) -> Result<(), Error> {
    let mut active_profile_changed = proxy.receive_active_profile_changed().await;
    let mut performance_degraded_changed = proxy.receive_performance_degraded_changed().await;
    let mut profiles_changed = proxy.receive_profiles_changed().await;
    let mut actions_changed = proxy.receive_actions_changed().await;
    let mut active_profile_holds_changed = proxy.receive_active_profile_holds_changed().await;

    tokio::spawn(async move {
        loop {
            let Some(power_profiles) = weak_power_profiles.upgrade() else {
                return;
            };

            tokio::select! {
                _ = cancel_token.cancelled() => {
                    debug!("Power Profiles monitoring cancelled");
                    return;
                }

                Some(change) = active_profile_changed.next() => {
                    if let Ok(new_profile) = change.get().await {
                        let profile = PowerProfile::from(new_profile.as_str());
                        power_profiles.active_profile.set(profile);
                    }
                }

                Some(change) = performance_degraded_changed.next() => {
                    if let Ok(new_degraded) = change.get().await {
                        let profile = PerformanceDegradationReason::from(new_degraded.as_str());
                        power_profiles.performance_degraded.set(profile);
                    }
                }

                Some(change) = profiles_changed.next() => {
                    if let Ok(new_profiles) = change.get().await {
                        let profiles = new_profiles
                            .into_iter()
                            .filter_map(|profile| Profile::try_from(profile).ok()).collect();
                        power_profiles.profiles.set(profiles);
                    }
                }

                Some(change) = actions_changed.next() => {
                    if let Ok(new_actions) = change.get().await {
                        power_profiles.actions.set(new_actions);
                    }
                }

                Some(change) = active_profile_holds_changed.next() => {
                    if let Ok(new_holds) = change.get().await {
                        let holds = new_holds
                            .into_iter()
                            .filter_map(|hold| ProfileHold::try_from(hold).ok()).collect();
                        power_profiles.active_profile_holds.set(holds);
                    }
                }
            }
        }
    });

    Ok(())
}
