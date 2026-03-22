use std::sync::{Arc, Mutex};

use derive_more::Debug;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use wayle_common::Property;

use crate::{
    backend::types::{CommandSender, EventSender},
    builder::BrightnessServiceBuilder,
    core::BacklightDevice,
    error::Error,
};

/// Backlight management service. See [crate-level docs](crate) for usage.
#[derive(Debug)]
pub struct BrightnessService {
    #[debug(skip)]
    pub(crate) command_tx: CommandSender,
    #[debug(skip)]
    pub(crate) event_tx: EventSender,
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) backend_handle: Mutex<Option<JoinHandle<Result<(), Error>>>>,

    /// Updates automatically as devices appear/disappear via udev.
    pub devices: Property<Vec<Arc<BacklightDevice>>>,

    /// Highest-priority device by [`BacklightType`] ordering.
    pub primary: Property<Option<Arc<BacklightDevice>>>,
}

impl BrightnessService {
    /// Returns `None` when no sysfs backlight devices exist.
    ///
    /// # Errors
    ///
    /// Returns error if backend initialization fails.
    #[instrument]
    pub async fn new() -> Result<Option<Arc<Self>>, Error> {
        Self::builder().build().await
    }

    /// Custom service configuration.
    pub fn builder() -> BrightnessServiceBuilder {
        BrightnessServiceBuilder::new()
    }
}

impl BrightnessService {
    pub(crate) fn set_backend_handle(&self, handle: JoinHandle<Result<(), Error>>) {
        if let Ok(mut guard) = self.backend_handle.lock() {
            *guard = Some(handle);
        }
    }
}

impl Drop for BrightnessService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();

        let handle = self
            .backend_handle
            .lock()
            .ok()
            .and_then(|mut opt| opt.take());

        if let Some(handle) = handle {
            handle.abort();
        }
    }
}
