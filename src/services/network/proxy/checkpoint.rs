//! NetworkManager Checkpoint interface.

use zbus::{proxy, zvariant::OwnedObjectPath};

/// Configuration and State Snapshot.
///
/// Represents a checkpoint of network configuration.
#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Checkpoint"
)]
pub(crate) trait Checkpoint {
    /// Array of object paths for devices which are part of this checkpoint.
    #[zbus(property)]
    fn devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    /// The timestamp (in milliseconds since the Unix epoch) when the checkpoint was created.
    #[zbus(property)]
    fn created(&self) -> zbus::Result<i64>;

    /// Timeout in seconds for automatic rollback, or zero.
    #[zbus(property)]
    fn rollback_timeout(&self) -> zbus::Result<u32>;
}
