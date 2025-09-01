use tokio_util::sync::CancellationToken;
use zbus::Connection;

pub(crate) struct SettingsParams<'a> {
    pub zbus_connection: &'a Connection,
}

pub(crate) struct LiveSettingsParams<'a> {
    pub zbus_connection: &'a Connection,
    pub cancellation_token: CancellationToken,
}
