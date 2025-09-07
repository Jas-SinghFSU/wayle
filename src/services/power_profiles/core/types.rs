use tokio_util::sync::CancellationToken;
use zbus::Connection;

pub(crate) struct PowerProfilesParams<'a> {
    pub connection: &'a Connection,
}

pub(crate) struct LivePowerProfilesParams<'a> {
    pub connection: &'a Connection,
    pub cancellation_token: &'a CancellationToken,
}
