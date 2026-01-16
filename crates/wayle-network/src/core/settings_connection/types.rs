use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

#[doc(hidden)]
pub struct ConnectionSettingsParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
}

#[doc(hidden)]
pub struct LiveConnectionSettingsParams<'a> {
    pub(crate) connection: &'a Connection,
    pub(crate) path: OwnedObjectPath,
    pub(crate) cancellation_token: &'a CancellationToken,
}
