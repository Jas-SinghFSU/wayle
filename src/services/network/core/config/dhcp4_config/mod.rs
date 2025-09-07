mod types;

use std::collections::HashMap;

pub(crate) use types::Dhcp4ConfigParams;
use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::{
    common::property::Property,
    network::{error::NetworkError, proxy::dhcp4_config::DHCP4ConfigProxy},
    traits::Static,
};

/// IPv4 DHCP Client State.
///
/// This corresponds to the org.freedesktop.NetworkManager.DHCP4Config interface which
/// provides access to configuration options returned by the IPv4 DHCP server.
#[derive(Debug, Clone)]
pub struct Dhcp4Config {
    /// D-Bus object path for this DHCP4 configuration
    pub object_path: Property<OwnedObjectPath>,

    /// Configuration options returned by the DHCP server.
    pub options: Property<HashMap<String, OwnedValue>>,
}

impl Static for Dhcp4Config {
    type Error = NetworkError;
    type Context<'a> = Dhcp4ConfigParams<'a>;

    async fn get(params: Self::Context<'_>) -> Result<Self, Self::Error> {
        Self::from_path(params.connection, params.path).await
    }
}

impl Dhcp4Config {
    async fn from_path(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Self, NetworkError> {
        let options = Self::fetch_options(connection, &path).await?;
        Ok(Self::from_options(path, options))
    }

    async fn fetch_options(
        connection: &Connection,
        path: &OwnedObjectPath,
    ) -> Result<HashMap<String, OwnedValue>, NetworkError> {
        let proxy = DHCP4ConfigProxy::new(connection, path)
            .await
            .map_err(NetworkError::DbusError)?;

        let options = proxy.options().await.map_err(NetworkError::DbusError)?;

        let mut converted = HashMap::new();
        for (key, value) in options {
            match OwnedValue::try_from(&value) {
                Ok(owned_value) => {
                    converted.insert(key, owned_value);
                }
                Err(_) => {
                    return Err(NetworkError::DataConversionFailed {
                        data_type: format!("DHCP4 option '{key}'"),
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
