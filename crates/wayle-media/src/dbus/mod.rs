//! D-Bus interface for the media service.
//!
//! Contains the server-side daemon interface and client-side proxy.

mod client;
mod server;

pub use client::MediaProxy;
pub(crate) use server::MediaDaemon;

/// D-Bus service name.
pub const SERVICE_NAME: &str = "com.wayle.Media1";

/// D-Bus object path.
pub const SERVICE_PATH: &str = "/com/wayle/Media";
