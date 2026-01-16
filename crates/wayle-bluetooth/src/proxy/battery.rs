use zbus::proxy;

#[proxy(interface = "org.bluez.Battery1", default_service = "org.bluez")]
pub(crate) trait Battery1 {
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<u8>;

    #[zbus(property)]
    fn source(&self) -> zbus::Result<String>;
}
