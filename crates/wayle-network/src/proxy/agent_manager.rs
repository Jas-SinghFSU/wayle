//! NetworkManager Agent Manager interface.

use zbus::proxy;

#[proxy(
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.AgentManager",
    default_path = "/org/freedesktop/NetworkManager/AgentManager"
)]
pub(crate) trait AgentManager {
    /// Called by secret Agents to register their ability to provide and save network secrets.
    ///
    /// # Arguments
    /// * `identifier` - Identifies this agent; only one agent in each user session may use the same identifier.
    ///   Identifier formatting follows the same rules as D-Bus bus names with the exception that the ':' character is not allowed.
    fn register(&self, identifier: &str) -> zbus::Result<()>;

    /// Like Register() but indicates agent capabilities to NetworkManager.
    ///
    /// # Arguments
    /// * `identifier` - See Register() for details
    /// * `capabilities` - NMSecretAgentCapabilities flags
    fn register_with_capabilities(&self, identifier: &str, capabilities: u32) -> zbus::Result<()>;

    /// Called by secret Agents to notify NetworkManager that they will no longer handle requests for network secrets.
    fn unregister(&self) -> zbus::Result<()>;
}
