use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

#[doc(hidden)]
pub struct WiredParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
}

#[doc(hidden)]
pub struct LiveWiredParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) device_path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
}
