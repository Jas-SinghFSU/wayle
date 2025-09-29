use std::collections::HashMap;

use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedValue};

/// Parameters for creating a PowerProfiles instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct PowerProfilesParams<'a> {
    pub(crate) connection: &'a Connection,
}

/// Parameters for creating a live PowerProfiles instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
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
