use std::{collections::HashMap, sync::Arc};

use zbus::{
    Connection,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::services::{
    common::Property,
    network::{NetworkError, proxy::dhcp4_config::DHCP4ConfigProxy},
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

impl Dhcp4Config {
    /// Get a snapshot of the current DHCP4 configuration state (no monitoring).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError::DbusError` if D-Bus operations fail or
    /// `NetworkError::DataConversionFailed` if DHCP option conversion fails.
    pub(crate) async fn get(
        connection: &Connection,
        path: OwnedObjectPath,
    ) -> Result<Arc<Self>, NetworkError> {
        let config = Self::from_path(connection, path).await?;
        Ok(Arc::new(config))
    }

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
        let proxy = DHCP4ConfigProxy::new(connection, path.clone())
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
                        reason: "Failed to convert to OwnedValue".to_string(),
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
