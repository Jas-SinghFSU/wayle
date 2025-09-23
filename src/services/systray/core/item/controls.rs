use zbus::Connection;

use crate::services::systray::{
    error::Error, proxy::status_notifier_item::StatusNotifierItemProxy,
};

pub(super) struct TrayItemController;

impl TrayItemController {
    pub async fn context_menu(
        connection: &Connection,
        bus_name: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::new(connection, bus_name).await?;

        proxy
            .context_menu(x, y)
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "context_menu",
                reason: err.to_string(),
            })
    }

    pub async fn activate(
        connection: &Connection,
        bus_name: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::new(connection, bus_name).await?;

        proxy
            .activate(x, y)
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "activate",
                reason: err.to_string(),
            })
    }

    pub async fn secondary_activate(
        connection: &Connection,
        bus_name: &str,
        x: i32,
        y: i32,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::new(connection, bus_name).await?;

        proxy
            .secondary_activate(x, y)
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "secondary_activate",
                reason: err.to_string(),
            })
    }

    pub async fn scroll(
        connection: &Connection,
        bus_name: &str,
        delta: i32,
        orientation: &str,
    ) -> Result<(), Error> {
        let proxy = StatusNotifierItemProxy::new(connection, bus_name).await?;

        proxy
            .scroll(delta, orientation)
            .await
            .map_err(|err| Error::OperationFailed {
                operation: "scroll",
                reason: err.to_string(),
            })
    }
}
