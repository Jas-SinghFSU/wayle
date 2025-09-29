use tracing::instrument;
use zbus::Connection;

use crate::{
    error::Error,
    proxy::power_profiles::PowerProfilesProxy,
    types::profile::{HoldCookie, PowerProfile, ProfileHold},
};

pub(super) struct PowerProfilesController;

impl PowerProfilesController {
    #[instrument(skip(connection), fields(profile = %profile), err)]
    pub async fn set_active_profile(
        connection: &Connection,
        profile: PowerProfile,
    ) -> Result<(), Error> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy
            .set_active_profile(&profile.to_string())
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "set_active_profile",
                reason: format!("Failed to set active profile: {err}"),
            })
    }

    #[instrument(
        skip(connection),
        fields(profile = %hold.profile, reason = %hold.reason, app_id = %hold.application_id),
        err
    )]
    pub async fn hold_profile(
        connection: &Connection,
        hold: ProfileHold,
    ) -> Result<HoldCookie, Error> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy
            .hold_profile(
                &hold.profile.to_string(),
                &hold.reason,
                &hold.application_id,
            )
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "hold_profile",
                reason: format!("Failed to hold profile: {err}"),
            })
    }

    #[instrument(skip(connection, hold_cookie), err)]
    pub async fn release_profile(
        connection: &Connection,
        hold_cookie: HoldCookie,
    ) -> Result<(), Error> {
        let proxy = PowerProfilesProxy::new(connection).await?;

        proxy
            .release_profile(hold_cookie)
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "release_profile",
                reason: format!("Failed to release profile: {err}"),
            })
    }
}
