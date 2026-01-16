use std::collections::HashMap;

use zbus::{Result, proxy, zvariant::OwnedValue};

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
pub(crate) trait PowerProfiles {
    async fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> Result<u32>;

    async fn release_profile(&self, cookie: u32) -> Result<()>;

    #[zbus(signal)]
    fn profile_released(&self, cookie: u32) -> Result<()>;

    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    #[zbus(property)]
    fn set_active_profile(&self, profile: &str) -> Result<()>;

    #[zbus(property)]
    fn performance_degraded(&self) -> Result<String>;

    #[zbus(property)]
    fn profiles(&self) -> Result<Vec<HashMap<String, OwnedValue>>>;

    #[zbus(property)]
    fn actions(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn active_profile_holds(&self) -> Result<Vec<HashMap<String, OwnedValue>>>;

    #[zbus(property)]
    fn version(&self) -> Result<String>;
}
