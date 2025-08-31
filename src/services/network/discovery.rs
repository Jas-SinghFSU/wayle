use zbus::{Connection, zvariant::OwnedObjectPath};

use super::{DeviceProxy, NMDeviceType, NetworkError, NetworkManagerProxy};
use crate::services::common::NULL_PATH;

pub(crate) struct NetworkServiceDiscovery;

impl NetworkServiceDiscovery {
    pub(crate) async fn wifi_device_path(
        connection: &Connection,
    ) -> Result<Option<OwnedObjectPath>, NetworkError> {
        Self::find_device_path(connection, NMDeviceType::Wifi, false).await
    }

    pub(crate) async fn wired_device_path(
        connection: &Connection,
    ) -> Result<Option<OwnedObjectPath>, NetworkError> {
        Self::find_device_path(connection, NMDeviceType::Ethernet, false).await
    }
    async fn find_device_path(
        connection: &Connection,
        target_type: NMDeviceType,
        prefer_active: bool,
    ) -> Result<Option<OwnedObjectPath>, NetworkError> {
        let nm_proxy = NetworkManagerProxy::new(connection).await?;
        let devices = nm_proxy
            .get_all_devices()
            .await
            .map_err(NetworkError::DbusError)?;

        let mut fallback = None;

        for path in devices {
            let device_proxy = DeviceProxy::new(connection, path.clone())
                .await
                .map_err(NetworkError::DbusError)?;

            let device_type = device_proxy
                .device_type()
                .await
                .map_err(NetworkError::DbusError)?;

            if device_type == target_type as u32 {
                if !prefer_active {
                    return Ok(Some(path));
                }

                let active = device_proxy
                    .active_connection()
                    .await
                    .map_err(NetworkError::DbusError)?;

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
