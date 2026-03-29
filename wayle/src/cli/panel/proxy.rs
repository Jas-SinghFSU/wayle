//! D-Bus proxy utilities for panel commands.

use std::time::Duration;

use futures::StreamExt;
use wayle_ipc::{
    shell::{APP_ID, GtkActionsProxy},
    shell_ipc::ShellIpcProxy,
};
use zbus::{Connection, Error as ZbusError, fdo::DBusProxy};

use crate::cli::dbus;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

/// Establishes a D-Bus session connection.
///
/// # Errors
///
/// Returns error if the session bus is unavailable.
pub async fn connect() -> Result<Connection, String> {
    Connection::session()
        .await
        .map_err(|e| format!("D-Bus session unavailable: {e}"))
}

/// Creates a proxy for the panel's GtkActions interface.
///
/// # Errors
///
/// Returns error if the proxy cannot be created.
pub async fn actions_proxy(connection: &Connection) -> Result<GtkActionsProxy<'_>, String> {
    GtkActionsProxy::new(connection)
        .await
        .map_err(|e| format!("Failed to connect to panel: {e}"))
}

/// Checks if wayle-shell is currently running via D-Bus name ownership.
///
/// # Errors
///
/// Returns error if D-Bus query fails.
pub async fn is_running() -> Result<bool, String> {
    let connection = connect().await?;

    let dbus = DBusProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create D-Bus proxy: {e}"))?;

    let name = APP_ID
        .try_into()
        .map_err(|e| format!("Invalid app ID: {e}"))?;

    dbus.name_has_owner(name)
        .await
        .map_err(|e| format!("Failed to query D-Bus: {e}"))
}

/// Waits for the panel's D-Bus name to be released.
///
/// Subscribes to NameOwnerChanged and waits until the name has no owner,
/// with a timeout to prevent hanging if shutdown fails.
///
/// # Errors
///
/// Returns error if subscription fails or timeout expires.
pub async fn wait_for_shutdown(connection: &Connection) -> Result<(), String> {
    let dbus = DBusProxy::new(connection)
        .await
        .map_err(|e| format!("Failed to create D-Bus proxy: {e}"))?;

    let mut name_changes = dbus
        .receive_name_owner_changed()
        .await
        .map_err(|e| format!("Failed to subscribe to name changes: {e}"))?;

    let wait_for_release = async {
        while let Some(signal) = name_changes.next().await {
            let Ok(args) = signal.args() else { continue };

            if args.name() == APP_ID && args.new_owner().is_none() {
                return Ok(());
            }
        }
        Err("Name change stream ended unexpectedly".to_string())
    };

    tokio::time::timeout(SHUTDOWN_TIMEOUT, wait_for_release)
        .await
        .map_err(|_| "Timeout waiting for panel to stop".to_string())?
}

/// Creates a ShellIpcProxy for shell commands.
///
/// # Errors
///
/// Returns error if D-Bus connection or proxy creation fails.
pub async fn shell_ipc_proxy() -> Result<(Connection, ShellIpcProxy<'static>), String> {
    let connection = connect().await?;

    let proxy = ShellIpcProxy::new(&connection)
        .await
        .map_err(|err| format!("cannot create shell IPC proxy: {err}"))?;

    Ok((connection, proxy))
}

/// Formats a shell IPC D-Bus error for CLI output.
pub fn format_ipc_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error("Shell", operation, error)
}
