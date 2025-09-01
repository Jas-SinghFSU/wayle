use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct ConnectionSettingsParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}

pub(crate) struct LiveConnectionSettingsParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
    pub cancellation_token: CancellationToken,
}
