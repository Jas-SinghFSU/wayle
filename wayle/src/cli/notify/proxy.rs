//! D-Bus proxy utilities for notification commands.

use wayle_notification::WayleNotificationsProxy;
use zbus::{Connection, Error as ZbusError};

use crate::cli::dbus;

const SERVICE_NAME: &str = "Notification";

/// Creates a WayleNotificationsProxy connection.
///
/// # Errors
/// Returns error if D-Bus connection or proxy creation fails.
pub async fn connect() -> Result<(Connection, WayleNotificationsProxy<'static>), String> {
    let connection = dbus::session().await?;

    let proxy = WayleNotificationsProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create notification proxy: {e}"))?;

    Ok((connection, proxy))
}

/// Transforms zbus errors into user-friendly messages.
pub fn format_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error(SERVICE_NAME, operation, error)
}
