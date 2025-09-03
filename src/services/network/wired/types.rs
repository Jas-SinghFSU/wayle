use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct WiredParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
}

pub(crate) struct LiveWiredParams<'a> {
    pub connection: &'a Connection,
    pub device_path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}
