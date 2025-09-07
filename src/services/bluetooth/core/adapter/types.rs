use tokio_util::sync::CancellationToken;
use zbus::{Connection, zvariant::OwnedObjectPath};

/// Context for static adapter operations
pub(crate) struct AdapterParams<'a> {
    /// D-Bus connection for adapter communication
    pub connection: &'a Connection,
    /// Adapter object path
    pub path: OwnedObjectPath,
}

/// Context for live adapter operations with monitoring
pub(crate) struct LiveAdapterParams<'a> {
    /// D-Bus connection for adapter communication
    pub connection: &'a Connection,
    /// Adapter object path
    pub path: OwnedObjectPath,
    /// Token for cancelling monitoring operations
    pub cancellation_token: &'a CancellationToken,
}

pub(crate) struct AdapterProperties {
    pub address: String,
    pub address_type: String,
    pub name: String,
    pub alias: String,
    pub class: u32,
    pub connectable: bool,
    pub powered: bool,
    pub power_state: String,
    pub discoverable: bool,
    pub discoverable_timeout: u32,
    pub discovering: bool,
    pub pairable: bool,
    pub pairable_timeout: u32,
    pub uuids: Vec<String>,
    pub modalias: Option<String>,
    pub roles: Vec<String>,
    pub experimental_features: Vec<String>,
    pub manufacturer: u16,
    pub version: u8,
}
