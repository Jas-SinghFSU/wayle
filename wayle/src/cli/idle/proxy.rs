use wayle_common::idle_inhibit::IdleInhibitProxy;
use zbus::{Connection, Error as ZbusError};

use crate::cli::dbus;

const SERVICE_NAME: &str = "IdleInhibit";

/// Creates an IdleInhibitProxy connection.
///
/// # Errors
/// Returns error if D-Bus connection or proxy creation fails.
pub async fn connect() -> Result<(Connection, IdleInhibitProxy<'static>), String> {
    let connection = dbus::session().await?;

    let proxy = IdleInhibitProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create idle inhibit proxy: {e}"))?;

    Ok((connection, proxy))
}

/// Transforms zbus errors into user-friendly messages.
pub fn format_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error(SERVICE_NAME, operation, error)
}
