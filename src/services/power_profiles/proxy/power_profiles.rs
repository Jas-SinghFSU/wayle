use std::collections::HashMap;

use zbus::{Result, proxy, zvariant::OwnedValue};

/// Proxy for the org.freedesktop.UPower.PowerProfiles interface.
///
/// The power-profiles-daemon API is meant to be used by parts of the OS or desktop
/// environment to switch system power profiles based on user choice, or user intent.
///
/// OS components would typically use the "Profiles" property to construct their UI
/// (2 or 3 profiles available), and monitor the "ActiveProfile" and the
/// "PerformanceDegraded" properties to update that UI. The UI would try to set the
/// "ActiveProfile" property if the user selected a different one.
///
/// Interface: `org.freedesktop.UPower.PowerProfiles`
/// Object path: `/org/freedesktop/UPower/PowerProfiles`
#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub(crate) trait PowerProfiles {
    /// This forces the passed profile (either 'power-saver' or 'performance') to be
    /// activated until either the caller quits, "ReleaseProfile" is called, or the
    /// "ActiveProfile" is changed by the user.
    ///
    /// This should be used programmatically by OS components when, eg.
    /// high-performance workloads are started with the "performance" profile, or
    /// battery will soon be critically low with the "power-saver" profile.
    ///
    /// When conflicting profiles are requested to be held, the 'power-saver' profile
    /// will be activated in preference to the 'performance' profile.
    ///
    /// Those holds will be automatically cancelled if the user manually switches to
    /// another profile, and the "ProfileReleased" signal will be emitted.
    ///
    /// # Arguments
    /// * `profile` - Profile to hold ('power-saver' or 'performance')
    /// * `reason` - Reason for holding the profile
    /// * `application_id` - Identifier of the application requesting the hold
    ///
    /// # Returns
    /// Token to release the hold later
    async fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> Result<u32>;

    /// This removes the hold that was set on a profile.
    ///
    /// # Arguments
    /// * `cookie` - The cookie returned by HoldProfile
    async fn release_profile(&self, cookie: u32) -> Result<()>;

    /// This signal will be emitted if the profile is released because the
    /// "ActiveProfile" was manually changed.
    ///
    /// # Arguments
    /// * `cookie` - The cookie of the released hold
    #[zbus(signal)]
    fn profile_released(&self, cookie: u32) -> Result<()>;

    /// The type of the currently active profile. It might change automatically if a
    /// profile is held, using the "HoldProfile" function.
    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    /// Sets the active profile.
    #[zbus(property)]
    fn set_active_profile(&self, profile: &str) -> Result<()>;

    /// This property is deprecated, and unused since version 0.9.
    #[zbus(property)]
    fn performance_inhibited(&self) -> Result<String>;

    /// This will be set if the performance power profile is running in degraded mode,
    /// with the value being used to identify the reason for that degradation. As new
    /// reasons can be added, it is recommended that front-ends show a generic reason
    /// if they do not recognise the value.
    #[zbus(property)]
    fn performance_degraded(&self) -> Result<String>;

    /// An array of key-pair values representing each profile.
    #[zbus(property)]
    fn profiles(&self) -> Result<Vec<HashMap<String, OwnedValue>>>;

    /// An array of strings listing each one of the "actions" implemented in the running
    /// daemon. This is used by API users to figure out whether particular
    /// functionality is available in a version of the daemon.
    #[zbus(property)]
    fn actions(&self) -> Result<Vec<String>>;

    /// A list of dictionaries representing the current profile holds.
    #[zbus(property)]
    fn active_profile_holds(&self) -> Result<Vec<HashMap<String, OwnedValue>>>;

    /// The version of the power-profiles-daemon software.
    #[zbus(property)]
    fn version(&self) -> Result<String>;
}
