use zbus::proxy;

/// Proxy for the org.bluez.AgentManager1 interface.
///
/// BlueZ D-Bus AgentManager API for managing pairing agents.
///
/// Service: `org.bluez`
/// Interface: `org.bluez.AgentManager1`
/// Object path: `/org/bluez`
#[proxy(
    interface = "org.bluez.AgentManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez"
)]
pub(crate) trait AgentManager1 {
    /// Registers pairing agent.
    ///
    /// The object path defines the path of the agent that will be called when user
    /// input is needed and must implement org.bluez.Agent1 interface.
    ///
    /// Every application can register its own agent and for all actions triggered by
    /// that application its agent is used.
    ///
    /// It is not required by an application to register an agent. If an application
    /// does chooses to not register an agent, the default agent is used. This is on
    /// most cases a good idea. Only application like a pairing wizard should register
    /// their own agent.
    ///
    /// An application can only register one agent. Multiple agents per application is
    /// not supported.
    ///
    /// Possible capability values:
    ///
    /// - `""`: Fallback to "KeyboardDisplay".
    /// - `"DisplayOnly"`
    /// - `"DisplayYesNo"`
    /// - `"KeyboardOnly"`
    /// - `"NoInputNoOutput"`
    /// - `"KeyboardDisplay"`
    ///
    /// # Errors
    ///
    /// - `InvalidArguments` - Invalid capability or agent path
    /// - `AlreadyExists` - Agent already registered
    async fn register_agent(
        &self,
        agent: &zbus::zvariant::ObjectPath<'_>,
        capability: &str,
    ) -> zbus::Result<()>;

    /// Unregisters an agent that has been previously registered using
    /// RegisterAgent. The object path parameter must match the same value that has
    /// been used on registration.
    ///
    /// # Errors
    ///
    /// - `DoesNotExist` - Agent not registered
    async fn unregister_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Requests to make the application agent the default agent. The application is
    /// required to register an agent.
    ///
    /// Special permission might be required to become the default agent.
    ///
    /// # Errors
    ///
    /// - `DoesNotExist` - Agent not registered
    async fn request_default_agent(
        &self,
        agent: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}
