use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::{
    WorkspaceId,
    ipc::{HyprMessenger, events::types::ServiceNotification},
};

#[doc(hidden)]
pub struct WorkspaceParams<'a> {
    pub(crate) id: WorkspaceId,
    pub(crate) hypr_messenger: &'a HyprMessenger,
}

#[doc(hidden)]
pub struct LiveWorkspaceParams<'a> {
    pub(crate) id: WorkspaceId,
    pub(crate) hypr_messenger: &'a HyprMessenger,
    pub(crate) internal_tx: &'a Sender<ServiceNotification>,
    pub(crate) cancellation_token: &'a CancellationToken,
}
