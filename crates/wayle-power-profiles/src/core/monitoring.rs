use std::sync::{Arc, Weak};

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};
use wayle_traits::ModelMonitoring;
use zbus::fdo::DBusProxy;

use super::PowerProfiles;
use crate::{
    error::Error,
    proxy::power_profiles::PowerProfilesProxy,
    types::profile::{PerformanceDegradationReason, PowerProfile, Profile, ProfileHold},
};

const PPD_BUS_NAME: &str = "org.freedesktop.UPower.PowerProfiles";

impl ModelMonitoring for PowerProfiles {
    type Error = Error;

    async fn start_monitoring(self: Arc<Self>) -> Result<(), Self::Error> {
        let proxy = PowerProfilesProxy::new(&self.zbus_connection).await?;
        let weak_self = Arc::downgrade(&self);
        let Some(ref cancellation_token) = self.cancellation_token else {
            return Err(Error::MissingCancellationToken);
        };

        monitor_property_changes(weak_self.clone(), proxy, cancellation_token.clone()).await?;
        monitor_daemon_lifecycle(weak_self, &self.zbus_connection, cancellation_token.clone())
            .await;

        Ok(())
    }
}

async fn monitor_property_changes(
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
                    debug!("Power Profiles property monitoring cancelled");
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

async fn monitor_daemon_lifecycle(
    weak_power_profiles: Weak<PowerProfiles>,
    connection: &zbus::Connection,
    cancel_token: CancellationToken,
) {
    let Ok(dbus_proxy) = DBusProxy::new(connection).await else {
        warn!("cannot create D-Bus proxy for ppd lifecycle monitoring");
        return;
    };

    let Ok(mut name_owner_changed) = dbus_proxy.receive_name_owner_changed().await else {
        warn!("cannot subscribe to NameOwnerChanged for ppd");
        return;
    };

    let connection = connection.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    debug!("Power Profiles lifecycle monitoring cancelled");
                    return;
                }

                Some(signal) = name_owner_changed.next() => {
                    let Ok(args) = signal.args() else { continue };

                    if args.name() != PPD_BUS_NAME {
                        continue;
                    }

                    let Some(power_profiles) = weak_power_profiles.upgrade() else {
                        return;
                    };

                    let daemon_appeared = args.old_owner().is_none() && args.new_owner().is_some();
                    let daemon_disappeared = args.old_owner().is_some() && args.new_owner().is_none();

                    if daemon_appeared {
                        info!("power-profiles-daemon appeared on D-Bus");
                        refresh_all_properties(&power_profiles, &connection).await;
                    } else if daemon_disappeared {
                        info!("power-profiles-daemon disappeared from D-Bus");
                        clear_all_properties(&power_profiles);
                    }
                }
            }
        }
    });
}

async fn refresh_all_properties(power_profiles: &PowerProfiles, connection: &zbus::Connection) {
    let Ok(proxy) = PowerProfilesProxy::new(connection).await else {
        warn!("cannot create proxy to refresh power profile properties");
        return;
    };

    if let Ok(active) = proxy.active_profile().await {
        power_profiles
            .active_profile
            .set(PowerProfile::from(active.as_str()));
    }

    if let Ok(degraded) = proxy.performance_degraded().await {
        power_profiles
            .performance_degraded
            .set(PerformanceDegradationReason::from(degraded.as_str()));
    }

    if let Ok(raw_profiles) = proxy.profiles().await {
        let profiles = raw_profiles
            .into_iter()
            .filter_map(|profile| Profile::try_from(profile).ok())
            .collect();
        power_profiles.profiles.set(profiles);
    }

    if let Ok(actions) = proxy.actions().await {
        power_profiles.actions.set(actions);
    }

    if let Ok(raw_holds) = proxy.active_profile_holds().await {
        let holds = raw_holds
            .into_iter()
            .filter_map(|hold| ProfileHold::try_from(hold).ok())
            .collect();
        power_profiles.active_profile_holds.set(holds);
    }
}

fn clear_all_properties(power_profiles: &PowerProfiles) {
    power_profiles.profiles.set(Vec::new());
    power_profiles.actions.set(Vec::new());
    power_profiles.active_profile_holds.set(Vec::new());
}
