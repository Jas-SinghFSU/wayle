use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

pub(crate) struct ActiveConnectionParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
}

pub(crate) struct LiveActiveConnectionParams<'a> {
    pub connection: &'a Connection,
    pub path: OwnedObjectPath,
    pub cancellation_token: &'a CancellationToken,
}
