use zbus::Connection;

use crate::services::notification::{
    error::Error,
    types::{
        ClosedReason, Signal,
        dbus::{SERVICE_INTERFACE, SERVICE_PATH},
    },
};

pub(super) struct NotificationControls;

impl NotificationControls {
    pub async fn dismiss(connection: &Connection, id: &u32) -> Result<(), Error> {
        connection
            .emit_signal(
                None::<()>,
                SERVICE_PATH,
                SERVICE_INTERFACE,
                Signal::NotificationClosed.as_str(),
                &(id, ClosedReason::DismissedByUser as u32),
            )
            .await?;

        Ok(())
    }

    pub async fn invoke(connection: &Connection, id: &u32, action_key: &str) -> Result<(), Error> {
        connection
            .emit_signal(
                None::<()>,
                SERVICE_PATH,
                SERVICE_INTERFACE,
                Signal::ActionInvoked.as_str(),
                &(id, action_key),
            )
            .await?;

        Ok(())
    }
}
