#![allow(missing_docs)]
use std::collections::HashMap;

use zbus::{
    Result, proxy,
    zvariant::{OwnedObjectPath, OwnedValue},
};

#[proxy(interface = "org.bluez.Device1", default_service = "org.bluez")]
pub(crate) trait Device1 {
    async fn connect(&self) -> Result<()>;

    async fn disconnect(&self) -> Result<()>;

    async fn connect_profile(&self, uuid: &str) -> Result<()>;

    async fn disconnect_profile(&self, uuid: &str) -> Result<()>;

    async fn pair(&self) -> Result<()>;

    async fn cancel_pairing(&self) -> Result<()>;

    async fn get_service_records(&self) -> Result<Vec<Vec<u8>>>;

    #[zbus(property)]
    fn address(&self) -> Result<String>;

    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    #[zbus(property)]
    fn name(&self) -> Result<String>;

    #[zbus(property)]
    fn icon(&self) -> Result<String>;

    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    #[zbus(property)]
    fn appearance(&self) -> Result<u16>;

    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;

    #[zbus(property)]
    fn paired(&self) -> Result<bool>;

    #[zbus(property)]
    fn bonded(&self) -> Result<bool>;

    #[zbus(property)]
    fn connected(&self) -> Result<bool>;

    #[zbus(property)]
    fn trusted(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_trusted(&self, trusted: bool) -> Result<()>;

    #[zbus(property)]
    fn blocked(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_blocked(&self, blocked: bool) -> Result<()>;

    #[zbus(property)]
    fn wake_allowed(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_wake_allowed(&self, allowed: bool) -> Result<()>;

    #[zbus(property)]
    fn alias(&self) -> Result<String>;

    #[zbus(property)]
    fn set_alias(&self, alias: &str) -> Result<()>;

    #[zbus(property)]
    fn adapter(&self) -> Result<OwnedObjectPath>;

    #[zbus(property)]
    fn legacy_pairing(&self) -> Result<bool>;

    #[zbus(property)]
    fn cable_pairing(&self) -> Result<bool>;

    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    #[zbus(property)]
    fn rssi(&self) -> Result<i16>;

    #[zbus(property)]
    fn tx_power(&self) -> Result<i16>;

    #[zbus(property)]
    fn manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>>;

    #[zbus(property)]
    fn service_data(&self) -> Result<HashMap<String, Vec<u8>>>;

    #[zbus(property)]
    fn services_resolved(&self) -> Result<bool>;

    #[zbus(property)]
    fn advertising_flags(&self) -> Result<Vec<u8>>;

    #[zbus(property)]
    fn advertising_data(&self) -> Result<HashMap<u8, Vec<u8>>>;

    #[zbus(property)]
    fn sets(&self) -> Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;

    #[zbus(property)]
    fn preferred_bearer(&self) -> Result<String>;

    #[zbus(property)]
    fn set_preferred_bearer(&self, bearer: &str) -> Result<()>;

    #[zbus(signal)]
    async fn disconnected(&self, reason: u8, message: String) -> Result<()>;
}
