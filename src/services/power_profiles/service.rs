use std::{ops::Deref, sync::Arc};

use tokio_util::sync::CancellationToken;
use tracing::instrument;
use zbus::Connection;

use super::{
    core::{PowerProfiles, types::LivePowerProfilesParams},
    error::PowerProfilesError,
};
use crate::services::traits::Reactive;

/// Power profiles service for managing system power profiles and monitoring changes.
///
/// Provides a high-level interface to the system's power profile daemon,
/// allowing access to available profiles, current active profile, and reactive
/// monitoring of profile changes through the D-Bus interface.
pub struct PowerProfilesService {
    power_profiles: Arc<PowerProfiles>,
}

impl Deref for PowerProfilesService {
    type Target = PowerProfiles;

    fn deref(&self) -> &Self::Target {
        &self.power_profiles
    }
}

impl PowerProfilesService {
    /// Creates a new power profiles service instance.
    ///
    /// Establishes a connection to the system D-Bus and initializes live monitoring
    /// of power profile changes. The service will automatically track profile state
    /// changes and provide reactive access to current profile information.
    ///
    /// # Errors
    ///
    /// Returns `PowerProfilesError::ServiceInitializationFailed` if service initialization
    /// fails.
    #[instrument]
    pub async fn new() -> Result<Self, PowerProfilesError> {
        let connection = Connection::system().await.map_err(|err| {
            PowerProfilesError::ServiceInitializationFailed(format!(
                "D-Bus connection failed: {err}"
            ))
        })?;

        let cancellation_token = CancellationToken::new();

        let power_profiles = PowerProfiles::get_live(LivePowerProfilesParams {
            connection: &connection,
            cancellation_token: &cancellation_token,
        })
        .await?;

        Ok(Self { power_profiles })
    }
}
