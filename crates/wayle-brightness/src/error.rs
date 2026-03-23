use std::io;

use zbus::Error as ZbusError;

/// Backlight operation failures.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Reading a sysfs attribute failed.
    #[error("cannot read sysfs backlight at {path}")]
    SysfsRead {
        /// Absolute path to the sysfs file.
        path: String,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Writing brightness to sysfs failed.
    #[error("cannot write brightness to {path}")]
    SysfsWrite {
        /// Absolute path to the sysfs file.
        path: String,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// logind D-Bus `SetBrightness` call failed.
    #[error("cannot set brightness via logind")]
    LogindSetFailed(#[source] ZbusError),

    /// logind session bus is unreachable.
    #[error("cannot connect to logind session bus")]
    LogindConnectionFailed(#[source] ZbusError),

    /// poll(POLLPRI) setup on sysfs brightness file failed.
    #[error("cannot watch brightness file at {path}")]
    WatchFailed {
        /// Absolute path to the sysfs file.
        path: String,
        /// Underlying filesystem error.
        #[source]
        source: io::Error,
    },

    /// Backend command channel closed unexpectedly.
    #[error("command channel disconnected")]
    CommandChannelDisconnected,

    /// System has no `/sys/class/backlight/` entries.
    #[error("no backlight devices found")]
    NoDevices,
}
