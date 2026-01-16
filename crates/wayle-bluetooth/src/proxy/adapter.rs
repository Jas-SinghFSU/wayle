use std::collections::HashMap;

use zbus::{
    Result, proxy,
    zvariant::{ObjectPath, OwnedObjectPath, Value},
};

#[proxy(interface = "org.bluez.Adapter1", default_service = "org.bluez")]
pub(crate) trait Adapter1 {
    async fn start_discovery(&self) -> Result<()>;

    async fn stop_discovery(&self) -> Result<()>;

    async fn remove_device(&self, device: &ObjectPath<'_>) -> Result<()>;

    async fn set_discovery_filter(&self, filter: HashMap<String, Value<'_>>) -> Result<()>;

    async fn get_discovery_filters(&self) -> Result<Vec<String>>;

    async fn connect_device(
        &self,
        properties: HashMap<String, Value<'_>>,
    ) -> Result<OwnedObjectPath>;

    #[zbus(property)]
    fn address(&self) -> Result<String>;

    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    #[zbus(property)]
    fn name(&self) -> Result<String>;

    #[zbus(property)]
    fn alias(&self) -> Result<String>;

    #[zbus(property)]
    fn set_alias(&self, alias: &str) -> Result<()>;

    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    #[zbus(property)]
    fn connectable(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_connectable(&self, connectable: bool) -> Result<()>;

    #[zbus(property)]
    fn powered(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_powered(&self, powered: bool) -> Result<()>;

    #[zbus(property)]
    fn power_state(&self) -> Result<String>;

    #[zbus(property)]
    fn discoverable(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_discoverable(&self, discoverable: bool) -> Result<()>;

    #[zbus(property)]
    fn pairable(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_pairable(&self, pairable: bool) -> Result<()>;

    #[zbus(property)]
    fn pairable_timeout(&self) -> Result<u32>;

    #[zbus(property)]
    fn set_pairable_timeout(&self, timeout: u32) -> Result<()>;

    #[zbus(property)]
    fn discoverable_timeout(&self) -> Result<u32>;

    #[zbus(property)]
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<()>;

    #[zbus(property)]
    fn discovering(&self) -> Result<bool>;

    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    #[zbus(property)]
    fn roles(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn experimental_features(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn manufacturer(&self) -> Result<u16>;

    #[zbus(property)]
    fn version(&self) -> Result<u8>;
}
