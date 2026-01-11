//! D-Bus interface for the power profiles service.
//!
//! Contains the Wayle daemon interface and client-side proxy.

mod client;
mod server;

pub use client::PowerProfilesWayleProxy;
pub(crate) use server::PowerProfilesDaemon;

/// D-Bus service name.
pub const SERVICE_NAME: &str = "com.wayle.PowerProfiles1";

/// D-Bus object path.
pub const SERVICE_PATH: &str = "/com/wayle/PowerProfiles";
