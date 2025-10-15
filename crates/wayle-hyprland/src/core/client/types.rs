use tokio_util::sync::CancellationToken;

use crate::{Address, ipc::HyprMessenger};

#[doc(hidden)]
pub struct ClientParams<'a> {
    pub(crate) address: Address,
    pub(crate) hypr_messenger: &'a HyprMessenger,
}

#[doc(hidden)]
pub struct LiveClientParams<'a> {
    pub(crate) address: Address,
    pub(crate) hypr_messenger: &'a HyprMessenger,
    pub(crate) cancellation_token: &'a CancellationToken,
}
