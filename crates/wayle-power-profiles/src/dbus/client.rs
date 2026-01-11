//! D-Bus client proxy for power profiles.
#![allow(missing_docs)]

use zbus::{Result, proxy};

/// D-Bus client proxy for controlling power profiles.
///
/// Connects to a running power profiles daemon and allows external control
/// of system power profiles.
#[proxy(
    interface = "com.wayle.PowerProfiles1",
    default_service = "com.wayle.PowerProfiles1",
    default_path = "/com/wayle/PowerProfiles",
    gen_blocking = false
)]
pub trait PowerProfilesWayle {
    /// Sets the active power profile.
    ///
    /// `profile` must be one of: "power-saver", "balanced", "performance".
    async fn set_profile(&self, profile: String) -> Result<()>;

    /// Cycles to the next power profile.
    async fn cycle(&self) -> Result<()>;

    /// Lists available power profiles.
    async fn list_profiles(&self) -> Result<Vec<String>>;

    /// Gets the active power profile.
    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    /// Gets the performance degradation reason if any.
    #[zbus(property)]
    fn performance_degraded(&self) -> Result<String>;

    /// Number of available profiles.
    #[zbus(property)]
    fn profile_count(&self) -> Result<u32>;
}
