mod types;

use std::sync::Arc;

use types::{LivePowerProfilesParams, PowerProfilesParams};

use crate::services::{common::property::Property, traits::Reactive};

use super::{
    error::PowerProfilesError,
    types::profile::{
        HoldCookie, PerformanceDegradationReason, PowerProfile, Profile, ProfileHold,
    },
};

/// Power profiles management with reactive properties.
///
/// Provides access to system power profiles through reactive Property fields
/// that automatically update when the underlying power-profiles-daemon state changes.
pub struct PowerProfiles {
    /// The type of the currently active profile.
    pub active_profile: Property<PowerProfile>,
    /// This will be set if the performance power profile is running in degraded mode.
    pub performance_degraded: Property<PerformanceDegradationReason>,
    /// An array of key-pair values representing each profile.
    pub profiles: Property<Vec<Profile>>,
    /// An array of strings listing each one of the "actions" implemented in the running daemon.
    pub actions: Vec<String>,
    /// A list of dictionaries representing the current profile holds.
    pub active_profile_holds: Vec<ProfileHold>,
}

impl Reactive for PowerProfiles {
    type Error = PowerProfilesError;
    type LiveContext<'a> = LivePowerProfilesParams<'a>;
    type Context<'a> = PowerProfilesParams<'a>;

    async fn get(_context: Self::Context<'_>) -> Result<Self, Self::Error> {
        todo!()
    }

    async fn get_live(_context: Self::LiveContext<'_>) -> Result<Arc<Self>, Self::Error> {
        todo!()
    }
}

impl PowerProfiles {
    /// Sets the active profile.
    ///
    /// # Errors
    /// Returns error if profile setting fails.
    pub fn set_active_profile(&self, _profile: PowerProfile) -> Result<(), PowerProfilesError> {
        todo!()
    }

    /// This forces the passed profile to be activated until either the caller quits,
    /// "ReleaseProfile" is called, or the "ActiveProfile" is changed by the user.
    ///
    /// # Errors
    /// Returns error if profile hold cannot be established.
    pub fn hold_profile(&self, _hold: ProfileHold) -> Result<HoldCookie, PowerProfilesError> {
        todo!()
    }

    /// This removes the hold that was set on a profile.
    ///
    /// # Errors
    /// Returns error if hold release fails or cookie is invalid.
    pub fn release_profile(
        &self,
        _hold_cookie: HoldCookie,
    ) -> Result<HoldCookie, PowerProfilesError> {
        todo!()
    }
}
