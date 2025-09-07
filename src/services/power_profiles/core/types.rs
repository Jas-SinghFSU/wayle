use std::collections::HashMap;

use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedValue};

pub(crate) struct PowerProfilesParams<'a> {
    pub connection: &'a Connection,
}

pub(crate) struct LivePowerProfilesParams<'a> {
    pub connection: &'a Connection,
    pub cancellation_token: &'a CancellationToken,
}

pub(crate) struct PowerProfilesProps {
    pub active_profile: String,
    pub performance_degraded: String,
    pub profiles: Vec<HashMap<String, OwnedValue>>,
    pub actions: Vec<String>,
    pub active_profile_holds: Vec<HashMap<String, OwnedValue>>,
}
