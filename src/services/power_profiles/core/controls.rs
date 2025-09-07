use zbus::Connection;

use crate::services::power_profiles::{
    error::PowerProfilesError,
    proxy::power_profiles::PowerProfilesProxy,
    types::profile::{HoldCookie, PowerProfile, ProfileHold},
};

pub(super) struct PowerProfilesController;

impl PowerProfilesController {
    pub async fn set_active_profile(
        connection: &Connection,
        profile: PowerProfile,
    ) -> Result<(), PowerProfilesError> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy
            .set_active_profile(&profile.to_string())
            .await
            .map_err(|err| PowerProfilesError::OperationFailed {
                operation: "set_active_profile",
                reason: format!("Failed to set active profile: {err}"),
            })
    }

    pub async fn hold_profile(
        connection: &Connection,
        hold: ProfileHold,
    ) -> Result<HoldCookie, PowerProfilesError> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy
            .hold_profile(
                &hold.profile.to_string(),
                &hold.reason,
                &hold.application_id,
            )
            .await
            .map_err(|err| PowerProfilesError::OperationFailed {
                operation: "hold_profile",
                reason: format!("Failed to hold profile: {err}"),
            })
    }

    pub async fn release_profile(
        connection: &Connection,
        hold_cookie: HoldCookie,
    ) -> Result<(), PowerProfilesError> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy.release_profile(hold_cookie).await.map_err(|err| {
            PowerProfilesError::OperationFailed {
                operation: "release_profile",
                reason: format!("Failed to release profile: {err}"),
            }
        })
    }
}
