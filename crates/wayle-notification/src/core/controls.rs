use tracing::instrument;
use zbus::Connection;

use crate::{
    error::Error,
    types::{
        Signal,
        dbus::{SERVICE_INTERFACE, SERVICE_PATH},
    },
};

pub(super) struct NotificationControls;

impl NotificationControls {
    #[instrument(skip(connection), fields(notification_id = %id, action = %action_key), err)]
    pub(super) async fn invoke(
        connection: &Connection,
        id: &u32,
        action_key: &str,
    ) -> Result<(), Error> {
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
