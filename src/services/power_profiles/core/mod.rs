mod controls;
mod monitoring;
/// Type definitions for power profiles core functionality
pub mod types;

use std::sync::Arc;

use controls::PowerProfilesController;
use derive_more::Debug;
use tokio_util::sync::CancellationToken;
use types::{LivePowerProfilesParams, PowerProfilesParams, PowerProfilesProps};
use zbus::Connection;

use super::{
    error::Error,
    proxy::power_profiles::PowerProfilesProxy,
    types::profile::{
        HoldCookie, PerformanceDegradationReason, PowerProfile, Profile, ProfileHold,
    },
};
use crate::{
    services::{
        common::Property,
        traits::{ModelMonitoring, Reactive},
    },
    unwrap_string, unwrap_vec,
};

/// Power profiles management with reactive properties.
///
/// Provides access to system power profiles through reactive Property fields
/// that automatically update when the underlying power-profiles-daemon state changes.
#[derive(Debug, Clone)]
pub struct PowerProfiles {
    #[debug(skip)]
    cancellation_token: Option<CancellationToken>,
    #[debug(skip)]
    zbus_connection: Connection,
    /// The type of the currently active profile.
    pub active_profile: Property<PowerProfile>,
    /// This will be set if the performance power profile is running in degraded mode.
    pub performance_degraded: Property<PerformanceDegradationReason>,
    /// An array of key-pair values representing each profile.
    pub profiles: Property<Vec<Profile>>,
    /// An array of strings listing each one of the "actions" implemented in the running daemon.
    pub actions: Property<Vec<String>>,
    /// A list of dictionaries representing the current profile holds.
    pub active_profile_holds: Property<Vec<ProfileHold>>,
}

impl Reactive for PowerProfiles {
    type Error = Error;
    type LiveContext<'a> = LivePowerProfilesParams<'a>;
    type Context<'a> = PowerProfilesParams<'a>;

    async fn get(context: Self::Context<'_>) -> Result<Self, Self::Error> {
        let power_profiles_props = Self::from_connection(context.connection).await?;
        Ok(Self::from_props(
            power_profiles_props,
            context.connection,
            None,
        ))
    }

    async fn get_live(context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        let power_profiles_props = Self::from_connection(context.connection).await?;
        let power_profiles = Self::from_props(
            power_profiles_props,
            context.connection,
            Some(context.cancellation_token.child_token()),
        );

        let power_profiles_arc = Arc::new(power_profiles);

        power_profiles_arc.clone().start_monitoring().await?;

        Ok(power_profiles_arc)
    }
}

impl PowerProfiles {
    /// Sets the active profile.
    ///
    /// # Errors
    /// Returns error if profile setting fails.
    pub async fn set_active_profile(&self, profile: PowerProfile) -> Result<(), Error> {
        PowerProfilesController::set_active_profile(&self.zbus_connection, profile).await
    }

    /// This forces the passed profile to be activated until either the caller quits,
    /// "ReleaseProfile" is called, or the "ActiveProfile" is changed by the user.
    ///
    /// # Errors
    /// Returns error if profile hold cannot be established.
    pub async fn hold_profile(&self, hold: ProfileHold) -> Result<HoldCookie, Error> {
        PowerProfilesController::hold_profile(&self.zbus_connection, hold).await
    }

    /// This removes the hold that was set on a profile.
    ///
    /// # Errors
    /// Returns error if hold release fails or cookie is invalid.
    pub async fn release_profile(&self, hold_cookie: HoldCookie) -> Result<(), Error> {
        PowerProfilesController::release_profile(&self.zbus_connection, hold_cookie).await
    }

    async fn from_connection(connection: &Connection) -> Result<PowerProfilesProps, Error> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        let (active_profile, performance_degraded, profiles, actions, active_profile_holds) = tokio::join!(
            proxy.active_profile(),
            proxy.performance_degraded(),
            proxy.profiles(),
            proxy.actions(),
            proxy.active_profile_holds()
        );

        Ok(PowerProfilesProps {
            active_profile: unwrap_string!(active_profile),
            performance_degraded: unwrap_string!(performance_degraded),
            profiles: unwrap_vec!(profiles),
            actions: unwrap_vec!(actions),
            active_profile_holds: unwrap_vec!(active_profile_holds),
        })
    }

    fn from_props(
        props: PowerProfilesProps,
        connection: &Connection,
        cancellation_token: Option<CancellationToken>,
    ) -> Self {
        Self {
            zbus_connection: connection.clone(),
            cancellation_token,
            active_profile: Property::new(PowerProfile::from(props.active_profile.as_str())),
            performance_degraded: Property::new(PerformanceDegradationReason::from(
                props.performance_degraded.as_str(),
            )),
            profiles: Property::new(
                props
                    .profiles
                    .into_iter()
                    .filter_map(|profile| Profile::try_from(profile).ok())
                    .collect(),
            ),
            actions: Property::new(props.actions),
            active_profile_holds: Property::new(
                props
                    .active_profile_holds
                    .into_iter()
                    .filter_map(|hold| ProfileHold::try_from(hold).ok())
                    .collect(),
            ),
        }
    }
}
