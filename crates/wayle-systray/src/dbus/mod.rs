//! D-Bus interface for the system tray service.
//!
//! Contains the Wayle daemon interface and client-side proxy.

mod client;
mod server;

pub use client::SystemTrayWayleProxy;
pub(crate) use server::SystemTrayDaemon;

/// D-Bus service name.
pub const SERVICE_NAME: &str = "com.wayle.SystemTray1";

/// D-Bus object path.
pub const SERVICE_PATH: &str = "/com/wayle/SystemTray";
