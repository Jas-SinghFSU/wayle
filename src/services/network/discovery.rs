use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{
    error::Error,
    proxy::{devices::DeviceProxy, manager::NetworkManagerProxy},
    types::device::NMDeviceType,
};
use crate::services::common::NULL_PATH;

pub(crate) struct NetworkServiceDiscovery;

impl NetworkServiceDiscovery {
    pub async fn wifi_device_path(
        connection: &Connection,
    ) -> Result<Option<OwnedObjectPath>, Error> {
        Self::find_device_path(connection, NMDeviceType::Wifi, false).await
    }

    pub async fn wired_device_path(
        connection: &Connection,
    ) -> Result<Option<OwnedObjectPath>, Error> {
        Self::find_device_path(connection, NMDeviceType::Ethernet, false).await
    }
    async fn find_device_path(
        connection: &Connection,
        target_type: NMDeviceType,
        prefer_active: bool,
    ) -> Result<Option<OwnedObjectPath>, Error> {
        let nm_proxy = NetworkManagerProxy::new(connection).await?;
        let devices = nm_proxy.get_all_devices().await.map_err(Error::DbusError)?;

        let mut fallback = None;

        for path in devices {
            let device_proxy = DeviceProxy::new(connection, path.clone())
                .await
                .map_err(Error::DbusError)?;

            let device_type = device_proxy.device_type().await.map_err(Error::DbusError)?;

            if device_type == target_type as u32 {
                if !prefer_active {
                    return Ok(Some(path));
                }

                let active = device_proxy
                    .active_connection()
                    .await
                    .map_err(Error::DbusError)?;

                let has_active_connection = active.as_str() != NULL_PATH;

                if has_active_connection {
                    return Ok(Some(path));
                }

                fallback = Some(path);
            }
        }

        Ok(fallback)
    }
}
