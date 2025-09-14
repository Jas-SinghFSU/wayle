use std::num::NonZeroU32;

use zbus::Connection;

use crate::services::notification::{
    error::Error,
    types::{
        ClosedReason, Signal,
        dbus::{INTERFACE, PATH},
    },
};

pub(super) struct NotificationControls;

impl NotificationControls {
    pub async fn dismiss(connection: &Connection, id: &NonZeroU32) -> Result<(), Error> {
        connection
            .emit_signal(
                None::<()>,
                PATH,
                INTERFACE,
                Signal::NotificationClosed.as_str(),
                &(id.get(), ClosedReason::DismissedByUser as u32),
            )
            .await?;

        Ok(())
    }

    pub async fn invoke(
        connection: &Connection,
        id: &NonZeroU32,
        action_key: &str,
    ) -> Result<(), Error> {
        connection
            .emit_signal(
                None::<()>,
                PATH,
                INTERFACE,
                Signal::ActionInvoked.as_str(),
                &(id.get(), action_key),
            )
            .await?;

        Ok(())
    }
}
