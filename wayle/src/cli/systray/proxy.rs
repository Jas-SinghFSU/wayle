//! D-Bus proxy utilities for system tray commands.

use wayle_systray::SystemTrayWayleProxy;
use zbus::{Connection, Error as ZbusError};

use crate::cli::dbus;

const SERVICE_NAME: &str = "System tray";

/// Creates a SystemTrayWayleProxy connection.
///
/// # Errors
/// Returns error if D-Bus connection or proxy creation fails.
pub async fn connect() -> Result<(Connection, SystemTrayWayleProxy<'static>), String> {
    let connection = dbus::session().await?;

    let proxy = SystemTrayWayleProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create systray proxy: {e}"))?;

    Ok((connection, proxy))
}

/// Transforms zbus errors into user-friendly messages.
pub fn format_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error(SERVICE_NAME, operation, error)
}
