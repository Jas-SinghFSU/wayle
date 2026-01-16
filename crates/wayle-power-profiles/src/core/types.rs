use std::collections::HashMap;

use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedValue};

#[doc(hidden)]
pub struct PowerProfilesParams<'a> {
    pub(crate) connection: &'a Connection,
}

#[doc(hidden)]
pub struct LivePowerProfilesParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) cancellation_token: &'a CancellationToken,
}

pub(crate) struct PowerProfilesProps {
    pub active_profile: String,
    pub performance_degraded: String,
    pub profiles: Vec<HashMap<String, OwnedValue>>,
    pub actions: Vec<String>,
    pub active_profile_holds: Vec<HashMap<String, OwnedValue>>,
}
