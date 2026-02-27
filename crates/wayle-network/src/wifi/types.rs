use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::core::settings::Settings;

#[doc(hidden)]
pub struct WifiParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
    pub(crate) settings: Arc<Settings>,
}

#[doc(hidden)]
pub struct LiveWifiParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
    pub(crate) settings: Arc<Settings>,
}
