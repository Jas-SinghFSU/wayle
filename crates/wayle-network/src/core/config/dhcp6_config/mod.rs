mod types;

use std::collections::HashMap;

pub(crate) use types::Dhcp6ConfigParams;
use wayle_common::Property;
use wayle_traits::Static;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::{error::Error, proxy::dhcp6_config::DHCP6ConfigProxy};

/// IPv6 DHCP Client State.
///
/// This corresponds to the org.freedesktop.NetworkManager.DHCP6Config interface which
/// provides access to configuration options returned by the IPv6 DHCP server.
#[derive(Debug, Clone)]
pub struct Dhcp6Config {
    /// D-Bus object path for this DHCP6 configuration
    pub object_path: Property<OwnedObjectPath>,

    /// Configuration options returned by the DHCP server.
    pub options: Property<HashMap<String, OwnedValue>>,
}

impl Static for Dhcp6Config {
    type Error = Error;
    type Context<'a> = Dhcp6ConfigParams<'a>;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.path).await
    }
}

impl Dhcp6Config {
    async fn from_path(connection: &Connection, path: OwnedObjectPath) -> Result<Self, Error> {
        let options = Self::fetch_options(connection, &path).await?;
        Ok(Self::from_options(path, options))
    }

    async fn fetch_options(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<HashMap<String, OwnedValue>, Error> {
        let proxy = DHCP6ConfigProxy::new(connection, path)
            .await
            .map_err(Error::DbusError)?;

        let options = proxy.options().await.map_err(Error::DbusError)?;

        let mut converted = HashMap::new();
        for (key, value) in options {
            match OwnedValue::try_from(&value) {
                Ok(owned_value) => {
                    converted.insert(key, owned_value);
                }
                Err(_) => {
                    return Err(Error::DataConversionFailed {
                        data_type: format!("DHCP6 option '{key}'"),
                        reason: String::from("Failed to convert to OwnedValue"),
                    });
                }
            }
        }
        Ok(converted)
    }

    fn from_options(path: OwnedObjectPath, options: HashMap<String, OwnedValue>) -> Self {
        Self {
            object_path: Property::new(path),
            options: Property::new(options),
        }
    }
}
