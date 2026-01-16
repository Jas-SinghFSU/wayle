use std::sync::Arc;

use derive_more::Debug;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use zbus::Connection;

use crate::{builder::PowerProfilesServiceBuilder, core::PowerProfiles, error::Error};

/// Entry point for power-profiles-daemon integration. See [crate-level docs](crate).
#[derive(Debug)]
pub struct PowerProfilesService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) _connection: Option<Connection>,

    /// Reactive power profile state and controls.
    pub power_profiles: Arc<PowerProfiles>,
}

impl PowerProfilesService {
    /// Creates a service with default configuration.
    ///
    /// For advanced options (e.g., D-Bus daemon registration), use [`Self::builder()`].
    ///
    /// # Errors
    ///
    /// Returns error if D-Bus connection or monitoring setup fails.
    #[instrument]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Returns a builder for advanced configuration.
    pub fn builder() -> PowerProfilesServiceBuilder {
        PowerProfilesServiceBuilder::new()
    }
}

impl Drop for PowerProfilesService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
