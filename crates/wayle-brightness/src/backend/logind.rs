use std::time::Duration;

use tokio::time::timeout;
use tracing::{debug, warn};
use zbus::Connection;

use super::sysfs;
use crate::Error;

const SESSION_PATH: &str = "/org/freedesktop/login1/session/auto";
const LOGIND_DEST: &str = "org.freedesktop.login1";
const SESSION_IFACE: &str = "org.freedesktop.login1.Session";
const DBUS_TIMEOUT: Duration = Duration::from_secs(5);

/// Sets brightness via logind D-Bus `SetBrightness` method.
///
/// Falls back to direct sysfs write if logind is unavailable.
pub(crate) async fn set_brightness(
    connection: &Option<Connection>,
    name: &str,
    value: u32,
) -> Result<(), Error> {
    let Some(connection) = connection else {
        debug!("logind unavailable, falling back to sysfs write");
        return sysfs::write_brightness(name, value);
    };

    let args = ("backlight", name, value);

    let result = timeout(
        DBUS_TIMEOUT,
        connection.call_method(
            Some(LOGIND_DEST),
            SESSION_PATH,
            Some(SESSION_IFACE),
            "SetBrightness",
            &args,
        ),
    )
    .await;

    match result {
        Ok(Ok(_)) => Ok(()),

        Ok(Err(err)) => {
            warn!(
                error = %err,
                device = name,
                "logind SetBrightness failed, falling back to sysfs"
            );
            sysfs::write_brightness(name, value)
        }

        Err(_) => {
            warn!(
                device = name,
                "logind SetBrightness timed out, falling back to sysfs"
            );
            sysfs::write_brightness(name, value)
        }
    }
}

/// Pings logind to verify availability.
///
/// Returns `None` on non-systemd systems.
pub(crate) async fn connect() -> Option<Connection> {
    let connection = Connection::system().await.ok()?;

    let ping = connection
        .call_method(
            Some(LOGIND_DEST),
            "/org/freedesktop/login1",
            Some("org.freedesktop.DBus.Peer"),
            "Ping",
            &(),
        )
        .await;

    if ping.is_err() {
        warn!("logind not responding, brightness writes will use sysfs");
        return None;
    }

    debug!("logind available on system bus");
    Some(connection)
}
