use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::ipc::{HyprMessenger, events::types::ServiceNotification};

#[doc(hidden)]
pub struct MonitorParams<'a> {
    pub(crate) name: String,
    pub(crate) hypr_messenger: &'a HyprMessenger,
}

#[doc(hidden)]
pub struct LiveMonitorParams<'a> {
    pub(crate) name: String,
    pub(crate) hypr_messenger: &'a HyprMessenger,
    pub(crate) internal_tx: &'a Sender<ServiceNotification>,
    pub(crate) cancellation_token: &'a CancellationToken,
}
