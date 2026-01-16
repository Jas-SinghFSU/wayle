use zbus::proxy;

#[proxy(
    interface = "org.bluez.AgentManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez"
)]
pub(crate) trait AgentManager1 {
    async fn register_agent(
        &self,
        agent: &zbus::zvariant::ObjectPath<'_>,
        capability: &str,
    ) -> zbus::Result<()>;

    async fn unregister_agent(&self, agent: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    async fn request_default_agent(
        &self,
        agent: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}
