use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct WifiParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
}

pub(crate) struct LiveWifiParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
    pub cancellation_token: CancellationToken,
}
