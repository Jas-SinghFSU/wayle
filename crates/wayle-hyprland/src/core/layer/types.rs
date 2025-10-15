use crate::{Address, ipc::HyprMessenger};

#[doc(hidden)]
pub struct LayerParams<'a> {
    pub(crate) address: Address,
    pub(crate) hypr_messenger: &'a HyprMessenger,
}
