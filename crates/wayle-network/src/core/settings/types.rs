use tokio_util::sync::CancellationToken;
use zbus::Connection;

#[doc(hidden)]
pub struct SettingsParams<'a> {
    pub(crate) zbus_connection: &'a Connection,
}

#[doc(hidden)]
pub struct LiveSettingsParams<'a> {
    pub(crate) zbus_connection: &'a Connection,
    pub(crate) cancellation_token: &'a CancellationToken,
}
