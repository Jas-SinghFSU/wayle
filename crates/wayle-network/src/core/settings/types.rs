use tokio_util::sync::CancellationToken;
use zbus::Connection;

/// Parameters for creating a Settings instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct SettingsParams<'a> {
    pub(crate) zbus_connection: &'a Connection,
}

/// Parameters for creating a LiveSettings instance.
///
/// **Note**: This type is exposed for trait implementation requirements
/// but should not be constructed directly by external consumers.
#[doc(hidden)]
pub struct LiveSettingsParams<'a> {
    pub(crate) zbus_connection: &'a Connection,
    pub(crate) cancellation_token: &'a CancellationToken,
}
