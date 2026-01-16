use zbus::proxy;

#[proxy(interface = "org.bluez.Agent1")]
pub(crate) trait Agent1 {
    async fn release(&self) -> zbus::Result<()>;

    async fn request_pin_code(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<String>;

    async fn display_pin_code(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        pincode: &str,
    ) -> zbus::Result<()>;

    async fn request_passkey(&self, device: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<u32>;

    async fn display_passkey(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        passkey: u32,
        entered: u16,
    ) -> zbus::Result<()>;

    async fn request_confirmation(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        passkey: u32,
    ) -> zbus::Result<()>;

    async fn request_authorization(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    async fn authorize_service(
        &self,
        device: &zbus::zvariant::ObjectPath<'_>,
        uuid: &str,
    ) -> zbus::Result<()>;

    async fn cancel(&self) -> zbus::Result<()>;
}
