use std::sync::Arc;

use derive_more::Debug;
use tokio_util::sync::CancellationToken;
use tracing::instrument;
use zbus::Connection;

use crate::{builder::PowerProfilesServiceBuilder, core::PowerProfiles, error::Error};

/// Power profiles service for managing system power profiles and monitoring changes.
///
/// Provides a high-level interface to the system's power profile daemon,
/// allowing access to available profiles, current active profile, and reactive
/// monitoring of profile changes through the D-Bus interface.
#[derive(Debug)]
pub struct PowerProfilesService {
    #[debug(skip)]
    pub(crate) cancellation_token: CancellationToken,
    #[debug(skip)]
    pub(crate) _connection: Option<Connection>,

    /// The power profiles D-Bus proxy for system power management operations.
    pub power_profiles: Arc<PowerProfiles>,
}

impl PowerProfilesService {
    /// Creates a new power profiles service instance with default configuration.
    ///
    /// Establishes a connection to the system D-Bus and initializes live monitoring
    /// of power profile changes. The service will automatically track profile state
    /// changes and provide reactive access to current profile information.
    ///
    /// For more control over initialization, use [`Self::builder()`].
    ///
    /// # Errors
    ///
    /// Returns error if D-Bus connection fails or service initialization fails.
    #[instrument]
    pub async fn new() -> Result<Arc<Self>, Error> {
        Self::builder().build().await
    }

    /// Returns a builder for configuring the power profiles service.
    ///
    /// The builder provides advanced configuration options such as enabling D-Bus
    /// daemon registration for CLI control.
    pub fn builder() -> PowerProfilesServiceBuilder {
        PowerProfilesServiceBuilder::new()
    }
}

impl Drop for PowerProfilesService {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
